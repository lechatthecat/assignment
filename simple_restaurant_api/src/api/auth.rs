use actix_web::{
    HttpResponse,
    Responder, HttpRequest, web, http::StatusCode
};
use bcrypt::verify;
use log::error;
use tokio_postgres::NoTls;
use serde::{Deserialize, Serialize};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use crate::{api::jwt::jwt, db::model::user::User};

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
        "SELECT name,password FROM users WHERE name = $1;",
        &[&req.name]
    ).await.unwrap();
    if rows.is_empty() {
        return HttpResponse::Unauthorized().finish();
    }
    let password = rows.get(0).unwrap().get::<_, String>("password");

    // check if the password is valid
    match verify(&req.password, &password) {
        Ok(_) => {
            // if valid, create jwt token
            match jwt::create_token(&req.name) {
                Ok(token) => {
                    // Create UserData instance with the necessary user information
                    let user_data = User {
                        name: req.name.clone(),
                        token: token,
                    };

                    HttpResponse::Ok().json(user_data)
                },
                Err(err) => {
                    error!("{}", err);
                    HttpResponse::InternalServerError().finish()
                },
            }
        },
        Err(err) => {
            error!("{}", err);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}

pub async fn current_user(req: HttpRequest) -> impl Responder {
    match jwt::verify(&req) {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(err) => {
            error!("{}", err);
            HttpResponse::new(StatusCode::UNAUTHORIZED)
        }
    }
}
