
use std::{time::SystemTime, collections::HashSet};

use actix_web::{
    HttpResponse,
    HttpResponseBuilder
};
use chrono::{Duration, Utc};
use tokio_postgres::{Transaction, Error, types::ToSql};

use crate::library::logger;

pub async fn check_existence_and_insert_order(
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

pub async fn check_existence_and_insert_orders(
    transaction: &mut Transaction<'_>,
    restaurant_table_id: i32,
    menu_ids: &mut Vec<i32>,
    user_name: String,
) -> Result<Result<u64, Error>, HttpResponseBuilder>
{
    // Execute a query using the connection from the pool
    // Before inserting, we will validate each id.
    let menu_rows_result = transaction.query(
        r#"
        SELECT
            *
        FROM
            menus
        WHERE
            menus.id = ANY($1);
        ;
        "#,
        &[menu_ids]
    ).await;
    let menu_seconds = match menu_rows_result {
        Ok(menu_rows) => {
            // Get Only distinctive IDs
            let mut seen = HashSet::new();
            let distinct_ids: Vec<_> = menu_ids.into_iter().filter(|x| seen.insert((*x).clone())).collect();

            if !menu_rows.is_empty() && menu_rows.len() == distinct_ids.len() && menu_ids.len() <= 100 {
                menu_rows.iter().zip(menu_ids.iter()).map(|menu_row|{
                    let seconds:i32 = menu_row.0.get("cook_time_seconds");
                    (*menu_row.1, seconds)
                }).collect::<Vec<(i32, i32)>>()
            } else {
                return Err(HttpResponse::BadRequest());
            }
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
    let current_datetime_utc = Utc::now();

    // Using a tuple to store values directly
    let menu_data: Vec<(i32, SystemTime)> = menu_seconds.iter()
        .map(|&(menu_id, seconds)| {
            let timestamp = (current_datetime_utc + Duration::seconds(seconds as i64)).into();
            (menu_id, timestamp)
        })
        .collect();

    // Construct the params vector without storing references to local variables
    let params: Vec<Box<dyn ToSql + Sync>> = menu_data.iter()
        .flat_map(|&(menu_id, timestamp)| {
            // To store i32 and SystemTime in a same Vec, we use box
            vec![
                Box::new(restaurant_table_id) as Box<dyn ToSql + Sync>,
                Box::new(menu_id),
                Box::new(user_id.clone()),
                Box::new(timestamp)
            ]
        })
        .collect();
    
    // Construct the value strings
    let values_strs: Vec<String> = (1..).step_by(4)
        .take(menu_data.len())
        .map(|i| format!("(${}, ${}, ${}, ${}, false)", i, i + 1, i + 2, i + 3))
        .collect();


    let values_string = values_strs.join(", ");
    
    let sql = format!(
        r#"
        INSERT INTO
            orders
        (restaurant_table_id, menu_id, checked_by_user_id, expected_cook_finish_time, is_served_by_staff)
        VALUES
        {}
        ;
        "#, values_string
    );

    type ToSqlSync = dyn ToSql + Sync;

    let params_refs: Vec<&ToSqlSync> = params.iter().map(|b| &**b).collect();
    let rows_result = transaction.execute(&sql, &params_refs[..]).await;
    
    Ok(rows_result)
}