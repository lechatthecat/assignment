use std::time::SystemTime;

use actix_web::{
    HttpResponse,
    Responder, web, HttpRequest
};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use chrono::{Duration, Utc};
use log::{error, warn};
use serde::{Deserialize, Serialize};
use tokio_postgres::{NoTls, Transaction, Error};

#[derive(Serialize, Deserialize, Debug)]
pub struct AddOrderRequest {
    restaurant_table_id: i32,
    menu_id: i32,
    user_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteOrderRequest {
    order_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CompleteOrderRequest {
    order_id: i64,
    user_name: String,
}

pub async fn add_order(
    _req: HttpRequest,
    order_req: web::Json<AddOrderRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let mut conn = pool.get().await.unwrap();

    // Start a transaction
    let mut transaction = conn.transaction().await.unwrap();
    let rows_result = check_existence_and_insert(
        &mut transaction,
        order_req.restaurant_table_id,
        order_req.menu_id,
        order_req.user_name.clone(),
    ).await;
    // Commit the transaction
    transaction.commit().await.unwrap();

    match rows_result {
        Ok(result) => {
            match result {
                Ok(_rows) => {
                    return HttpResponse::NoContent();
                }
                Err(e) => {
                    error!("{}", e);
                    return HttpResponse::InternalServerError();
                }
            }
        },
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError();
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
            return HttpResponse::NoContent();
        },
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError();
        }
    };
}

pub async fn complete_order(
    order_req: web::Json<CompleteOrderRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
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
        &[&order_req.user_name]
    ).await;
    let user_id: i32 = match user_row_result {
        Ok(user_row) => user_row.get("id"),
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError();
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
            return HttpResponse::NoContent();
        },
        Err(err) => {
            error!("{}", err);
            return HttpResponse::InternalServerError();
        }
    };
}

async fn check_existence_and_insert(
    transaction: &mut Transaction<'_>,
    restaurant_table_id: i32,
    menu_id: i32,
    user_name: String,
) -> Result<Result<u64, Error>, String>
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
            error!("{}", err);
            return Err(err.to_string());
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
            error!("{}", err);
            return Err(err.to_string());
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
        Ok(_tables_rows) => {}
        Err(err) => {
            error!("{}", err);
            return Err(err.to_string());
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
