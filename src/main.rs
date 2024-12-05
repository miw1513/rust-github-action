use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    dotenv().ok();
    let greeting = env::var("GREETING").unwrap_or_else(|_| "Hello, World!".to_string());
    HttpResponse::Ok().body(greeting)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}