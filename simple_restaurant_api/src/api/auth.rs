use actix_web::{
    HttpResponse,
    Responder, HttpRequest, web, http::StatusCode
};
use bcrypt::verify;
use tokio_postgres::NoTls;
use serde::{Deserialize, Serialize};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use crate::{
    api::jwt::jwt,
    db::model::user::User,
    library::logger
};

#[derive(Serialize, Deserialize, Debug)]
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
    let rows = conn.query(
        "SELECT id,name,password FROM users WHERE name = $1;",
        &[&req.name]
    ).await.unwrap();
    if rows.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }
    let password: String = rows.get(0).unwrap().get("password");
    let user_id: i32 = rows.get(0).unwrap().get("id");
    // check if the password is valid
    match verify(&req.password, &password) {
        Ok(_) => {
            // if valid, create jwt token
            match jwt::create_token(&req.name) {
                Ok(token) => {
                    // Create UserData instance with the necessary user information
                    let user_data = User {
                        id: user_id,
                        name: req.name.clone(),
                        token: token,
                    };

                    HttpResponse::Ok().json(user_data)
                },
                Err(err) => {
                    logger::log(logger::Header::ERROR, &err.to_string());
                    HttpResponse::InternalServerError().finish()
                },
            }
        },
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn current_user(req: HttpRequest) -> impl Responder {
    match jwt::verify(&req) {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(err) => {
            logger::log(logger::Header::ERROR, &err.to_string());
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}
