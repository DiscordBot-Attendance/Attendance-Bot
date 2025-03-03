use std::env;

use crate::api::adapters::controllers::attendance_controller::show_member_attendance_handler;
use crate::api::adapters::controllers::team_controller::show_teams_handler;
use crate::api::adapters::controllers::{
    auth_controller::login, member_controller::show_member_handler,
};
use crate::config::database::establish_connection;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;

// Check if the API is running
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("API is running!")
}

pub async fn start_api() {
    // Load environment variables from .env
    dotenv().ok();

    // Get the API server address from .env or default to "127.0.0.1:8080"
    let api_address =
        env::var("API_SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    // Set up the database connection pool
    let db_pool = establish_connection(); // Assuming this returns a `DBPool`

    // Wrap the pool in `web::Data`
    let db_pool_data = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool_data.clone()) // Pass the wrapped pool to the app
            .route("/", web::get().to(health_check))
            .route("/login", web::post().to(login))
            .route(
                "/teams/{admin_discord_id}",
                web::get().to(show_teams_handler),
            )
            .route("/members/{team_name}", web::get().to(show_member_handler))
            .route("/attendance/{team_name}", web::get().to(show_member_attendance_handler))
    })
    .bind(&api_address)
    .expect("Failed to bind API server")
    .run();

    log::info!("API Server running on http://{}", api_address);
    server.await.expect("Error running API server");
}
