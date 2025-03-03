mod api;
mod bot;
mod config;
mod schema;

use api::adapters::http_server::{self};
use bcrypt::verify;
use bot::adapters::discord_bot::Handler;
use config::{database, logger, settings};
use serenity::prelude::*;
use tokio::task;

#[tokio::main]
async fn main() {
    // Initialize Logger
    logger::init_logger();
    log::info!("Starting Attendance Bot...");

    // Load App Configuration
    let config = settings::Settings::new();
    log::info!("Configuration loaded!");

    // Connect to the Database
    let db_pool = database::establish_connection();
    log::info!("Database connection established!");

    // run discord bot and actix api in parallel
    let bot_discord = task::spawn(run_discord_bot(db_pool.clone(), config.discord_token));
    let api_discord = task::spawn(run_api_server());

    // wait for both task complete
    let _ = tokio::join!(bot_discord, api_discord);
}

// function to start the discord bot
async fn run_discord_bot(
    db_pool: diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>,
    token: String,
) {
    // Initialize Discord Client
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler::new(db_pool.clone()))
        .await
        .expect("Error creating Discord client");

    // Start the Bot
    if let Err(e) = client.start().await {
        log::error!("Client error: {:?}", e);
    }
}

// function to start the API server
async fn run_api_server() {
    http_server::start_api().await;
}
