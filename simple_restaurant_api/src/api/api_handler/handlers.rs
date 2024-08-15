use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use super::super::auth;
use super::super::order;
use super::super::restaurant_table;
use super::super::menu;

async fn api_handler(req: HttpRequest) -> Result<HttpResponse> {
    // For 404
    let path = req.path();
    Ok(HttpResponse::NotFound().body(format!("This API: '{}' does not exist.", path)))
}

pub fn api_scope() -> Scope {
    // APIs except "login" and "current_user" are protected by JWT middleware
    web::scope("/api")
        .route("/auth/login", web::post().to(auth::login))
        .route("/auth/current_user", web::get().to(auth::current_user))
        .route("/table", web::get().to(restaurant_table::get_tables))
        .route("/table/{restaurant_table_id}/order", web::get().to(restaurant_table::get_table_orders))
        .route("/table/order", web::delete().to(restaurant_table::delete_orders))
        .route("/order/{order_id}", web::get().to(order::get_order))
        .route("/order", web::post().to(order::add_order))
        .route("/orders", web::post().to(order::add_orders))
        .route("/order", web::delete().to(order::delete_order))
        .route("/order/complete", web::delete().to(order::complete_order))
        .route("/menu", web::get().to(menu::get_menus))
        .route("/menu/{menu_id}", web::get().to(menu::get_menu))
        .default_service(web::route().to(api_handler)) // catch-all route for /api
}
