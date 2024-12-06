use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use std::env;

#[get("/")]
async fn index() -> impl Responder {
    dotenv().ok();
    let env = env::var("ENV").ok().unwrap();
    let endpoint = env::var("ENDPOINT").ok().unwrap();
    let port = env::var("PORT").ok().unwrap();

// Return the formatted response including environment details
HttpResponse::Ok().body(format!(
    "Server is running on environment: {},  endpoint: {}, port: {}",
    env, endpoint, port
))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}