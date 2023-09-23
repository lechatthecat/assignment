use actix_web::{
    web::Data, App, HttpServer
};

mod api;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the configuration object
    let pool = db::pool::get_db_pool().await;

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(api::api_handler::handlers::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
