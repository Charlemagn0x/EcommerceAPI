use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::env;
use dotenv::dotenv;

mod controllers {
    pub mod product_controller;
    pub mod user_controller;
    pub mod order_controller;
}

async fn create_product() -> impl Responder {
    match controllers::product_controller::create_product().await {
        Ok(_) => HttpResponse::Ok().body("Product created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn get_product() -> impl Responder {
    match controllers::product_controller::get_product().await {
        Ok(product) => HttpResponse::Ok().json(product),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

async fn update_product() -> impl Responder {
    match controllers::product_controller::update_product().await {
        Ok(_) => HttpResponse::Ok().body("Product updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn delete_product() -> impl Responder {
    match controllers::product_controller::delete_product().await {
        Ok(_) => HttpResponse::Ok().body("Product deleted successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn register_user() -> impl Responder {
    match controllers::user_controller::register_user().await {
        Ok(_) => HttpResponse::Ok().body("User registered successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn login_user() -> impl Responder {
    match controllers::user_controller::login_user().await {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::Unauthorized().body(e.to_string()),
    }
}

async fn update_profile() -> impl Responder {
    match controllers::user_controller::update_profile().await {
        Ok(_) => HttpResponse::Ok().body("Profile updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn create_order() -> impl Responder {
    match controllers::order_controller::create_order().await {
        Ok(_) => HttpResponse::Ok().body("Order created successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

async fn view_order() -> impl Responder {
    match controllers::order_controller::view_order().await {
        Ok(order) => HttpResponse::Ok().json(order),
        Err(e) => HttpResponse::NotFound().body(e.to_string()),
    }
}

async fn update_order_status() -> impl Responder {
    match controllers::order_controller::update_order_status().await {
        Ok(_) => HttpResponse::Ok().body("Order status updated successfully"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS not set in .env file");

    println!("Starting server at: {}", &server_address);

    HttpServer::new(|| {
        App::new()
            .route("/products", web::post().to(create_product))
            .route("/products/{id}", web::get().to(get_product))
            .route("/products/{id}", web::put().to(update_product))
            .route("/products/{id}", web::delete().to(delete_product))
            .route("/users/register", web::post().to(register_user))
            .route("/users/login", web::post().to(login_user))
            .route("/users/update", web::put().to(update_profile))
            .route("/orders", web::post().to(create_order))
            .route("/orders/{id}", web::get().to(view_order))
            .route("/orders/{id}/status", web::put().to(update_order_status))
    })
    .bind(server_address)?
    .run()
    .await
}