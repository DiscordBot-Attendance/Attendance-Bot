use std::env;

use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!test" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Hello World!").await {
                println!("Error message: {e:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // get the bot token
    dotenv().ok();
    let token = env::var("BOT_TOKEN").expect("Expect a token in .env file!");
    // set gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // create new bot instance as a client
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start bot
    if let Err(e) = client.start().await {
        println!("Client error: {e:?}");
    }
}
