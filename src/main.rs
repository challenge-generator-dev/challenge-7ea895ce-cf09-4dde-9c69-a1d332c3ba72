pub mod domain;

use actix_web::web;
use crate::domain::infrastructure::routes::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}