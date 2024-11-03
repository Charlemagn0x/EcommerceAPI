use actix_web::{App, HttpServer, web::Data};
use std::env;
use dotenv::dotenv;

mod routes;
mod db;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = db::establish_connection(&database_url).await.expect("Failed to connect to database");

    let server_port = env::var("SERVER_PORT").unwrap_or_else(|_| "8000".to_string());
    
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .configure(routes::init_routes)
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .run()
    .await
}