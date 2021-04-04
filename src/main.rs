
use actix_web::{HttpServer, App, middleware};

mod controllers;
mod infrastructure;
mod domains;
mod helpers;
mod services;

use controllers::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    HttpServer::new(||
            App::new()
                .configure(config)
                .wrap(middleware::Logger::default())

        )

        .bind("0.0.0.0:8080")?
        .run()
        .await


}