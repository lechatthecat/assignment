use actix_web::{
    web, HttpRequest, HttpResponse,
    Result, Scope
};
use super::super::auth;
use super::super::test;

async fn api_handler(req: HttpRequest) -> Result<HttpResponse> {
    // For 404
    let path = req.path();
    Ok(HttpResponse::Ok().body(format!("API call to {}", path)))
}

pub fn api_scope() -> Scope {
    web::scope("/api")
        .route("/hello/{name}", web::get().to(test::greet))
        .route("/auth/login", web::post().to(auth::login)) // register other API routes here
        .route("/auth/logout", web::post().to(auth::logout))
        .default_service(web::route().to(api_handler)) // catch-all route for /api
}
