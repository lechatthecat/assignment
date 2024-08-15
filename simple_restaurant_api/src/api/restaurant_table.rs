use std::time::SystemTime;

use actix_web::{
    HttpResponse,
    Responder, HttpRequest, web
};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;
use crate::{db::model::restaurant_table::{RestaurantTable, RestaurantTableOrder}, library::logger};

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderRequest {
    restaurant_table_id: i32,
}

pub async fn get_tables(
    _req: HttpRequest,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    // This API fetches all contents from "restaurant_tables" table.
    // If millions of restaurant table records are there, it might lead to memory depletion
    // But I believe such case won't happen.. if it does, it is a really big restaurant.

    // Execute a query using the connection from the pool
    let rows_result = conn.query(
        "SELECT id,table_number,note FROM restaurant_tables;",
        &[]
    ).await;
    match rows_result {
        Ok(rows) => {
            if rows.is_empty() {
                return HttpResponse::Ok().json(Vec::<RestaurantTable>::new());
            }
            return HttpResponse::Ok().json(rows.iter().map(|row| {
                RestaurantTable {
                    id: row.get("id"),
                    table_number: row.get("table_number"),
                    note: row.get("note"),
                }
            }).collect::<Vec<RestaurantTable>>());
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

pub async fn get_table_orders(
    restaurant_table_id: web::Path<i32>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();
    // Execute a query using the connection from the pool
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
            rt.id = $1 AND odr.deleted_at is null;
        ;
        "#,
        &[&restaurant_table_id.clone()]
    ).await;
    // Check the result of select SQL
    match rows_result {
        // Converting the result to vec
        Ok(rows) => {
            if rows.is_empty() {
                return HttpResponse::Ok().json(Vec::<RestaurantTableOrder>::new());
            }
            return HttpResponse::Ok().json(rows.iter().map(|row| {
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
                RestaurantTableOrder {
                    id: row.get("restaurant_table_id"),
                    table_number: row.get("table_number"),
                    table_note: row.get("table_note"),
                    price: row.get("price"),
                    menu_name: row.get("menu_name"),
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
            }).collect::<Vec<RestaurantTableOrder>>());
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}

pub async fn delete_orders(
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
            restaurant_table_id = $1
        ;
        "#,
        &[
            &order_req.restaurant_table_id,
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
