mod bot;
mod config;
mod schema;

use bot::adapters::discord_bot::Handler;
use config::{database, logger, settings};
use serenity::prelude::*;

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

    // Initialize Discord Client
    let token = config.discord_token;
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
