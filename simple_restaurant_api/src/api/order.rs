use std::time::SystemTime;

use actix_web::{
    HttpResponse,
    Responder, web, HttpRequest, HttpResponseBuilder, http::StatusCode
};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use chrono::{Duration, Utc, NaiveDateTime};
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Transaction, Error};

use crate::{
    db::model::restaurant_table::RestaurantTableOrder,
    library::logger
};

use super::jwt::jwt;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrderRequest {
    restaurant_table_id: i32,
    menu_id: i32,
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
    let rows_result = check_existence_and_insert(
        &mut transaction,
        order_req.restaurant_table_id,
        order_req.menu_id,
        user.sub.clone(),
    ).await;
    // Commit the transaction
    transaction.commit().await.unwrap();

    match rows_result {
        Ok(result) => {
            match result {
                Ok(_rows) => {
                    return HttpResponse::Ok().finish();
                }
                Err(e) => {
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
            return HttpResponse::BadRequest().finish();
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

async fn check_existence_and_insert(
    transaction: &mut Transaction<'_>,
    restaurant_table_id: i32,
    menu_id: i32,
    user_name: String,
) -> Result<Result<u64, Error>, HttpResponseBuilder>
{
    // Execute a query using the connection from the pool
    // Before inserting, we will validate each id.
    let menu_row_result = transaction.query_one(
        r#"
        SELECT
            *
        FROM
            menus
        WHERE
            menus.id = $1
        ;
        "#,
        &[&menu_id]
    ).await;
    let menu_seconds = match menu_row_result {
        Ok(menu_row) => {
            let seconds: i32 = menu_row.get("cook_time_seconds");
            seconds
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return Err(HttpResponse::BadRequest());
        }
    };
    let users_rows_result = transaction.query_one(
        r#"
        SELECT
            *
        FROM
            users
        WHERE
            users.name = $1
        ;
        "#,
        &[&user_name]
    ).await;
    let user_id: i32 = match users_rows_result {
        Ok(users_row) => {
            users_row.get("id")
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return Err(HttpResponse::BadRequest());
        }
    };
    let tables_row_result = transaction.query_one(
        r#"
        SELECT
            *
        FROM
            restaurant_tables
        WHERE
            restaurant_tables.id = $1
        ;
        "#,
        &[&restaurant_table_id]
    ).await;
    match tables_row_result {
        Ok(_tables_row) => {}
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return Err(HttpResponse::BadRequest());
        }
    };
    // each id should be ok. inserting.
    let current_datetime_utc = Utc::now();

    let expected_cook_finish_time = current_datetime_utc + Duration::seconds(menu_seconds as i64);
    let timestamp: SystemTime = expected_cook_finish_time.into();

    let rows_result = transaction.execute(
        r#"
        INSERT INTO
            orders
        (restaurant_table_id, menu_id, checked_by_user_id, expected_cook_finish_time, is_served_by_staff)
            values
        ($1, $2, $3, $4, false)
        ;
        "#,
        &[
            &restaurant_table_id,
            &menu_id,
            &user_id,
            &timestamp
        ]
    ).await;
    Ok(rows_result)
}
