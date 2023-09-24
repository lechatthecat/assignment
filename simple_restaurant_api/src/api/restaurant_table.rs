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
use log::error;
use tokio_postgres::NoTls;
use crate::db::model::restaurant_table::{RestaurantTable, RestaurantTableOrders};

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
            error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };
}

pub async fn get_table_orders(
    table_id: web::Path<i32>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();
    // Execute a query using the connection from the pool
    let rows_result = conn.query(
        r#"
        SELECT
            restaurant_tables.id as restaurant_table_id,
            restaurant_tables.table_number as table_number,
            restaurant_tables.note as table_note,
            menus.name as menu_name,
            menus.cook_time_seconds as cook_time_seconds,
            orders.id as order_id,
            orders.expected_cook_finish_time as expected_cook_finish_time,
            orders.created_at as ordered_time,
            orders.is_served_by_staff as is_served_by_staff,
            orders.served_by_user_id as served_by_user_id,
            serve_user.name as serve_staff_name,
            orders.checked_by_user_id as checked_by_user_id,
            check_user.name as check_staff_name
        FROM
            restaurant_tables
        LEFT JOIN
            orders
        ON
            restaurant_tables.id = orders.restaurant_table_id
        LEFT JOIN
            menus
        ON
            orders.menu_id = menus.id
        LEFT JOIN
            users as serve_user
        ON
            orders.served_by_user_id = serve_user.id
        LEFT JOIN
            users as check_user
        ON
            orders.checked_by_user_id = check_user.id
        WHERE
            restaurant_tables.id = $1 and orders.deleted_at is null
        ;
        "#,
        &[&table_id.into_inner()]
    ).await;
    // Check the result of select SQL
    match rows_result {
        // Converting the result to vec
        Ok(rows) => {
            if rows.is_empty() {
                return HttpResponse::Ok().json(Vec::<RestaurantTableOrders>::new());
            }
            return HttpResponse::Ok().json(rows.iter().map(|row| {
                let expected_cook_finish_time: SystemTime = row.get("ordered_time");
                let expected_cook_finish_time = NaiveDateTime::from_timestamp_opt(
                    expected_cook_finish_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                );
                let ordered_time : SystemTime = row.get("ordered_time");
                let ordered_time = NaiveDateTime::from_timestamp_opt(
                    ordered_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64,
                    0,
                );
                RestaurantTableOrders {
                    id: row.get("restaurant_table_id"),
                    table_number: row.get("table_number"),
                    table_note: row.get("table_note"),
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
            }).collect::<Vec<RestaurantTableOrders>>());
        },
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };
}
