use actix_cors::Cors;
use actix_web::{
    web::Data, App, HttpServer
};
use api::middleware::jwt_middleware;

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
            .allowed_headers(vec!["Authorization", "Content-Type"])
            .max_age(60 * 60 * 24); // 1 days
            /*
            This sets the max_age to 1 day, meaning that
            once the browser makes a successful preflight request to the server,
            it can cache the results of that request for up to 3 days.
            Subsequent requests to the same resource within this time frame won't trigger another preflight request;
            the browser will use the cached results instead.
            */

        App::new()
            .wrap(jwt_middleware::JwtMiddleware)
            .wrap(cors)
            .app_data(Data::new(pool.clone()))
            .service(api::api_handler::handlers::api_scope())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
