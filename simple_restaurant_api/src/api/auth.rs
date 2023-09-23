use actix_web::{
    HttpResponse,
    Responder, HttpRequest, cookie::Cookie, web, http::StatusCode
};
use bcrypt::verify;
use serde::{Deserialize, Serialize};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use tokio_postgres::NoTls;
use crate::api::jwt::jwt;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    name: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    token: String
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
    if rows.len() == 0 {
        return HttpResponse::Unauthorized().finish();
    }
    let password = rows.get(0).unwrap().get::<_, String>(1);

    // check if the password is valid
    if verify(&req.password, &password).unwrap() {
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
            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

pub async fn current_user(req: HttpRequest) -> impl Responder {
    // Extract the token from the Authorization header
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // The token is typically prefixed by "Bearer" in the Authorization header
            let parts: Vec<&str> = auth_str.split_whitespace().collect();
            if parts.len() == 2 && parts[0] == "Bearer" {
                let token = parts[1];
                // Verify the token and decode the user information
                match jwt::decode_token(token) {
                    Ok(user_info) => {
                        let user_info = user_info.claims;
                        // Return user information if the token is valid
                        return HttpResponse::Ok().json(user_info);
                    },
                    Err(_) => {
                        // Token is invalid
                        return HttpResponse::new(StatusCode::UNAUTHORIZED);
                    }
                }
            }
        }
    }

    // No valid Authorization header found, the user is not logged in
    HttpResponse::new(StatusCode::UNAUTHORIZED)
}
