use actix_web::{
    HttpResponse,
    web,
    Responder
};
use bb8_postgres::{
    PostgresConnectionManager,
    bb8::Pool
};
use tokio_postgres::NoTls;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    #[serde(skip)]
    pub password: String,
    #[serde(skip)]
    pub refresh_token: Option<String>
}

// impl User {
//     pub fn get_all(
//         pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
//     ) -> Vec<User> {



//     }
// }
