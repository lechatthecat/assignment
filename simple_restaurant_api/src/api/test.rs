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

pub async fn greet(
    name: web::Path<String>,
    pool: web::Data<Pool<PostgresConnectionManager<NoTls>>>
) -> impl Responder {
    // Get a connection from the pool
    let conn = pool.get().await.unwrap();

    // Execute a query using the connection from the pool
    let rows = conn.query("SELECT * FROM restaurant_tables", &[]).await.unwrap();

    // for row in rows {
    //     let value: String = row.get(0); // Adjust the index and type according to your table
    //     println!("Value: {}", value);
    // }

    let value = match rows.get(0).unwrap().get::<_, Option<String>>(1) {
        Some(value) => value,
        None => String::from("Default Value"), // or handle it some other way
    };

    HttpResponse::Ok().body(format!("Hello, World! {}", value))
}
