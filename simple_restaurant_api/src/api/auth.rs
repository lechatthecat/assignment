use actix_web::{
    HttpResponse,
    Responder, HttpRequest, cookie::Cookie, post, web
};
use bcrypt::{hash, verify};
use serde::Deserialize;
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use serde_json::json;
use tokio_postgres::NoTls;
use crate::api::jwt::jwt;
use crate::db::model::user;

#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
    password: String,
}

pub async fn login(
    req: web::Json<LoginRequest>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    // Execute a query using the connection from the pool
    let rows = conn.query("SELECT name,password FROM users WHERE name = $1;", &[&req.name]).await.unwrap();
    if rows.len() == 0 {
        return HttpResponse::Unauthorized().finish();
    }
    let password = rows.get(0).unwrap().get::<_, String>(1);
    let hashed_password = hash(&password, 4).unwrap();

    // check if the password is valid
    if verify(&req.password, &hashed_password).unwrap() {
        // if valid, create jwt token
        match jwt::create_token(&req.name) {
            Ok(token) => {
                HttpResponse::Ok().json(json!({"token": token}))
            },
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub async fn logout(req: HttpRequest) -> impl Responder {
    // Directly deleting the cookie representing user login.
    HttpResponse::Ok()
        .cookie(Cookie::named("login_token"))
        .body("User logged out")
}
