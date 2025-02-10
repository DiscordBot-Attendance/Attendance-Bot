use std::env;

use dotenvy::dotenv;


// struct environment
pub struct Settings {
    pub discord_token: String,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();

        Self{
            discord_token: env::var("BOT_TOKEN").expect("BOT_TOKEN is not set in .env file!"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file!"),
        }
    }
}
