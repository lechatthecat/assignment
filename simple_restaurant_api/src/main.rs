use actix_cors::Cors;
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
        let cors = Cors::default()
            .allow_any_origin() // TODO For development purposes only, adjust for production
            // .allowed_origin("http://localhost:3001")
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(60 * 60 * 24 * 3); // 3 days

        App::new()
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .service(api::api_handler::handlers::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
