use actix_web::{App, HttpServer, web::Data, middleware::Logger};
use std::env;
use dotenv::dotenv;

mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => panic!("DATABASE_URL must be set. Error: {}", e),
    };
    
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting the server...");

    let pool = match db::establish_connection(&database_url).await {
        Ok(pool) => pool,
        Err(e) => panic!("Failed to connect to database. Error: {}", e),
    };

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind(format!("0.0.0.0:{}", server_port))
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("Failed to bind server: {}", e)))?
    .run()
    .await
}