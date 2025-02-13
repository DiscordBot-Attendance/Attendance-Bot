use crate::bot::infrastructure::persistence::user_repository;
use crate::config::database::establish_connection;
use crate::{bot::application::services::auth_service, config::database::DBPool};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use crate::config::constant::HELP_MESSAGES;

pub struct Handler {
    db_pool: DBPool,
}

impl Handler {
    pub fn new(db_pool: DBPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let conn = establish_connection();

        // show help message
        if msg.content.starts_with("!AB help") {
            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGES).await {
                println!("Error sending message: {e:?}");
            }
        }

        // register user
        if msg.content.starts_with("!AB register") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() < 3 {
                msg.channel_id
                    .say(&ctx.http, "Usage: !AB register <password>")
                    .await
                    .unwrap();
                return;
            }

            // get password
            let password = args[2];

            // db connection
            let mut db_conn = conn
                .get()
                .map_err(|_| "Failed to get DB connection")
                .unwrap();

            // save the user
            match user_repository::register_admin(
                &mut db_conn,
                &msg.author.id.to_string(),
                &msg.author.name,
                password,
            ) {
                Ok(_) => msg
                    .channel_id
                    .say(&ctx.http, "Admin registered successfully!")
                    .await
                    .unwrap(),
                Err(_) => msg
                    .channel_id
                    .say(&ctx.http, "Registration failed!")
                    .await
                    .unwrap(),
            };
        }

        // login user
        if msg.content.starts_with("!AB login") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() < 3 {
                msg.channel_id
                    .say(&ctx.http, "Usage: !AB login <password>")
                    .await
                    .unwrap();
            }

            // get password
            let password = args[2];

            // db connection
            let mut db_conn = conn
                .get()
                .map_err(|_| "Failed to get DB connection")
                .unwrap();

            match user_repository::authentication_admin(
                &mut db_conn,
                &msg.author.id.to_string(),
                password,
            ) {
                Ok(true) => {
                    let token = auth_service::generate_token(&msg.author.id.to_string());
                    msg.channel_id
                        .say(
                            &ctx.http,
                            format!("Login successful! Your token: {}", token),
                        )
                        .await
                        .unwrap()
                }
                _ => msg
                    .channel_id
                    .say(&ctx.http, "Invalid credentials!")
                    .await
                    .unwrap(),
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
