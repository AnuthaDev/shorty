mod db;
mod handlers;
mod models;
mod routes;
mod schema;
mod utils;

use actix_web::{middleware, web, App, HttpServer};
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get configuration from environment variables
    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("SERVER_PORT must be a valid port number");

    // Create database connection pool
    let pool = db::establish_connection_pool();

    log::info!("Starting server at http://{}:{}", host, port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
