use actix_files::Files;
use actix_web::{web, App, HttpServer};
use icfpc2024::www;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| String::from("0.0.0.0"));
    let server_port = env::var("PORT").unwrap_or_else(|_| String::from("8080"));
    let bind_address = format!("{}:{}", server_address, server_port);

    eprintln!("Starting server at: {}", bind_address);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(www::handlers::index))
            .service(Files::new("/", "/www"))
    })
    .bind(bind_address)?
    .run()
    .await
}
