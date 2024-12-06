use actix_web::{get, App, HttpResponse, HttpServer, Responder};
// use dotenv::dotenv;
// use std::env;

#[get("/")]
async fn index() -> impl Responder {
    // dotenv().ok();
    // let env = env::var("ENV").ok().unwrap();
    // let env = env::var("ENDPOINT").ok().unwrap();
    // let env = env::var("PORT").ok().unwrap();
    HttpResponse::Ok().body("test")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}