use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;


// check is api running or not
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is running!")
}

pub async fn start_api() {    
    // Load environment variables from .env
    dotenv().ok();

    // Get the API server address from .env or default to "127.0.0.1:8080"
    let api_address = env::var("API_SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());


    let server = HttpServer::new(|| {
        App::new().route("/", web::get().to(health_check))
    })
    .bind(&api_address)
    .expect("Failed to bind API server")
    .run();

     log::info!("API Server running on http://{}", api_address);
     server.await.expect("Error running API server");
}

