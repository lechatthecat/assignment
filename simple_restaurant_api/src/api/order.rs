use std::time::SystemTime;

use actix_web::{
    HttpResponse,
    Responder, web, HttpRequest, http::StatusCode
};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

use crate::{
    db::model::restaurant_table::RestaurantTableOrder,
    library::logger
};

use super::jwt::jwt;
use super::service;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrderRequest {
    restaurant_table_id: i32,
    menu_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrdersRequest {
    restaurant_table_id: i32,
    menu_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderRequest {
    order_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteOrderRequest {
    order_id: i64,
}

pub async fn get_order(
    _req: HttpRequest,
    order_id: web::Path<i32>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    let rows_result = conn.query(
        r#"
        SELECT
            rt.id as restaurant_table_id,
            rt.table_number as table_number,
            rt.note as table_note,
            mn.name as menu_name,
            mn.price as price,
            mn.cook_time_seconds as cook_time_seconds,
            odr.id as order_id,
            odr.expected_cook_finish_time as expected_cook_finish_time,
            odr.created_at as ordered_time,
            odr.is_served_by_staff as is_served_by_staff,
            odr.served_by_user_id as served_by_user_id,
            serve_user.name as serve_staff_name,
            odr.checked_by_user_id as checked_by_user_id,
            check_user.name as check_staff_name
        FROM
            restaurant_tables as rt
        INNER JOIN
            orders as odr
        ON
            rt.id = odr.restaurant_table_id
        INNER JOIN
            menus as mn
        ON
            odr.menu_id = mn.id
        LEFT JOIN
            users as serve_user
        ON
            odr.served_by_user_id = serve_user.id
        INNER JOIN
            users as check_user
        ON
            odr.checked_by_user_id = check_user.id
        WHERE
            odr.id = $1 AND odr.deleted_at is null;
        ;
        "#,
        &[&(order_id.into_inner() as i64)]
    ).await;

    match rows_result {
        Ok(rows) => {
            if rows.is_empty() {
                return HttpResponse::Ok().json("");
            }
            let row = rows.get(0).unwrap();
            let expected_cook_finish_time: Option<SystemTime> = row.get("expected_cook_finish_time");
            let expected_cook_finish_time = if let Some(data) = expected_cook_finish_time {
                NaiveDateTime::from_timestamp_opt(
                    data.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                )
            } else {
                None
            };
            let ordered_time : Option<SystemTime> = row.get("ordered_time");
            let ordered_time = if let Some(data) = ordered_time {
                NaiveDateTime::from_timestamp_opt(
                    data.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                )
            } else {
                None
            };
            return HttpResponse::Ok().json(
                RestaurantTableOrder {
                    id: row.get("restaurant_table_id"),
                    table_number: row.get("table_number"),
                    table_note: row.get("table_note"),
                    menu_name: row.get("menu_name"),
                    price: row.get("price"),
                    cook_time_seconds: row.get("cook_time_seconds"),
                    order_id: row.get("order_id"),
                    expected_cook_finish_time: expected_cook_finish_time,
                    ordered_time: ordered_time,
                    is_served_by_staff: row.get("is_served_by_staff"),
                    served_by_user_id: row.get("served_by_user_id"),
                    serve_staff_name: row.get("serve_staff_name"),
                    checked_by_user_id: row.get("checked_by_user_id"),
                    check_staff_name: row.get("check_staff_name"),
                }
            );
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

pub async fn add_order(
    req: HttpRequest,
    order_req: web::Json<AddOrderRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let user = match jwt::verify(&req) {
        Ok(user_info) => user_info,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };

    // Get a connection from the pool
    let mut conn = pool.get().await.unwrap();

    // Start a transaction
    let mut transaction = conn.transaction().await.unwrap();
    let rows_result = service::order_service::check_existence_and_insert_order(
        &mut transaction,
        order_req.restaurant_table_id,
        order_req.menu_id,
        user.sub.clone(),
    ).await;
    match rows_result {
        Ok(result) => {
            match result {
                Ok(_rows) => {
                    // Commit the transaction
                    transaction.commit().await.unwrap();
                    return HttpResponse::Ok().finish();
                }
                Err(e) => {
                    transaction.rollback().await.unwrap();
                    logger::log(logger::Header::ERROR, &e.to_string());
                    return HttpResponse::InternalServerError().finish();
                }
            }
        },
        Err(mut err) => {
            return err.finish();
        }
    };
}

pub async fn add_orders(
    req: HttpRequest,
    mut order_req: web::Json<AddOrdersRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let user = match jwt::verify(&req) {
        Ok(user_info) => user_info,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };

    // Get a connection from the pool
    let mut conn = pool.get().await.unwrap();

    // Start a transaction
    let mut transaction = conn.transaction().await.unwrap();
    let rows_result = service::order_service::check_existence_and_insert_orders(
        &mut transaction,
        order_req.restaurant_table_id,
        &mut order_req.menu_ids,
        user.sub.clone(),
    ).await;

    match rows_result {
        Ok(result) => {
            match result {
                Ok(_rows) => {
                    // Commit the transaction
                    transaction.commit().await.unwrap();
                    return HttpResponse::Ok().finish();
                }
                Err(e) => {
                    transaction.rollback().await.unwrap();
                    logger::log(logger::Header::ERROR, &e.to_string());
                    return HttpResponse::InternalServerError().finish();
                }
            }
        },
        Err(mut err) => {
            return err.finish();
        }
    };
}

pub async fn delete_order(
    _req: HttpRequest,
    order_req: web::Json<DeleteOrderRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let mut conn = pool.get().await.unwrap();

    // Start a transaction
    let transaction = conn.transaction().await.unwrap();

    let rows_result = transaction.execute(
        r#"
        UPDATE
            orders
        SET
            deleted_at = now()
        WHERE
            orders.id = $1
        ;
        "#,
        &[
            &order_req.order_id,
        ]
    ).await;

    // Commit the transaction
    transaction.commit().await.unwrap();

    match rows_result {
        Ok(_result) => {
            return HttpResponse::Ok();
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError();
        }
    };
}

pub async fn complete_order(
    req: HttpRequest,
    order_req: web::Json<CompleteOrderRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    let user = match jwt::verify(&req) {
        Ok(user_info) => user_info,
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::new(StatusCode::UNAUTHORIZED);
        }
    };

    // Get a connection from the pool
    let mut conn = pool.get().await.unwrap();

    // Start a transaction
    let transaction = conn.transaction().await.unwrap();
    let user_row_result = transaction.query_one(
        r#"
        SELECT id
        FROM users
        WHERE name = $1
        "#,
        &[&user.sub]
    ).await;
    let user_id: i32 = match user_row_result {
        Ok(user_row) => user_row.get("id"),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::BadRequest().json(format!("This user doesn\'t exist."));
        }
    };

    let rows_result = transaction.execute(
        r#"
        UPDATE
            orders
        SET
            deleted_at = now(),
            served_by_user_id = $2,
            is_served_by_staff = true
        WHERE
            orders.id = $1
        ;
        "#,
        &[
            &order_req.order_id,
            &user_id,
        ]
    ).await;
    // Commit the transaction
    transaction.commit().await.unwrap();

    match rows_result {
        Ok(_result) => {
            return HttpResponse::Ok().finish();
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}
