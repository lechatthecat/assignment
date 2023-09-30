use actix_web::{
    HttpResponse,
    Responder, HttpRequest, web
};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use tokio_postgres::NoTls;
use crate::{
    db::model::menu::Menu,
    library::logger
};

pub async fn get_menus(
    _req: HttpRequest,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    // This API fetches all contents from "menus" table.
    // If millions of restaurant menu records are there, it might lead to memory depletion
    // But I believe having millions of menu items won't happen..

    // Execute a query using the connection from the pool
    let rows_result = conn.query(
        "SELECT id,name,cook_time_seconds,price FROM menus;",
        &[]
    ).await;
    match rows_result {
        Ok(rows) => {
            if rows.is_empty() {
                return HttpResponse::Ok().json(Vec::<Menu>::new());
            }
            return HttpResponse::Ok().json(rows.iter().map(|row| {
                Menu {
                    id: row.get("id"),
                    name: row.get("name"),
                    cook_time_seconds: row.get("cook_time_seconds"),
                    price: row.get("price"),
                }
            }).collect::<Vec<Menu>>());
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}


pub async fn get_menu(
    _req: HttpRequest,
    menu_id: web::Path<i32>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    // Execute a query using the connection from the pool
    let rows_result = conn.query(
        "SELECT id,name,cook_time_seconds,price FROM menus WHERE id = $1;",
        &[&menu_id.into_inner()]
    ).await;
    match rows_result {
        Ok(rows) => {
            if rows.is_empty() {
                return HttpResponse::Ok().json("");
            }
            let row = rows.get(0).unwrap();
            return HttpResponse::Ok().json(
                Menu {
                    id: row.get("id"),
                    name: row.get("name"),
                    cook_time_seconds: row.get("cook_time_seconds"),
                    price: row.get("price"),
                }
            );
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            return HttpResponse::InternalServerError().finish();
        }
    };
}