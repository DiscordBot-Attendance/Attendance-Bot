use crate::bot::application::services::team_service;
use crate::bot::domain::model::User;
use crate::bot::infrastructure::persistence::user_repository;
use crate::config::database::establish_connection;
use crate::{bot::application::services::auth_service, config::database::DBPool};
use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::ExpressionMethods;
use diesel::RunQueryDsl;
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

        // create team
        if msg.content.starts_with("!AB create_team") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() < 3 {
                msg.channel_id
                    .say(&ctx.http, "Usage: !AB create_team <team_name>")
                    .await
                    .unwrap();
                return;
            }

            let team_name = args[2];

            // db connection
            let mut db_conn = conn
                .get()
                .map_err(|_| "Failed to get DB connection")
                .unwrap();

            // get admin id
            use crate::schema::users::dsl::*;
            let dc_user_id = msg.author.id.to_string();
            let admin_result: Result<User, diesel::result::Error> = users
                .filter(discord_id.eq(dc_user_id))
                .first::<User>(&mut db_conn);

            match admin_result {
                Ok(admin) => match team_service::register_team(&mut db_conn, team_name, admin.id) {
                    Ok(_) => {
                        msg.channel_id
                            .say(
                                &ctx.http,
                                format!("Team '{}' registered successfully!", team_name),
                            )
                            .await
                            .unwrap();
                    }
                    Err(_) => {
                        msg.channel_id
                            .say(&ctx.http, "Failed to register team.")
                            .await
                            .unwrap();
                    }
                },
                Err(diesel::NotFound) => {
                    msg.channel_id
                        .say(&ctx.http, "User not found!")
                        .await
                        .unwrap();
                }
                Err(e) => {
                    println!("Database error: {:?}", e);
                    msg.channel_id
                        .say(&ctx.http, "An error occurred while fetching user.")
                        .await
                        .unwrap();
                }
            };
        }

        // add member
        if msg.content.starts_with("!AB add_member") {
            let args: Vec<&str> = msg.content.split_whitespace().collect();
            if args.len() < 4 {
                msg.channel_id
                    .say(&ctx.http, "Usage: /admin add-member <team_name> @user")
                    .await
                    .unwrap();
                return;
            }

            // db connection
            let mut db_conn = conn
                .get()
                .map_err(|_| "Failed to get DB connection")
                .unwrap();

            let user_id = args[3].replace("<@", "").replace(">", "");

            // get team id
            use crate::schema::teams::dsl::*;
            let team_name = args[2];
            let team_result: Result<i32, diesel::result::Error> = teams
                .filter(name.eq(team_name))
                .select(id)
                .first::<i32>(&mut db_conn);

            match team_result {
                Ok(team_id) => {
                    match team_service::add_member(&mut db_conn, &user_id, team_id) {
                        Ok(_) => msg
                            .channel_id
                            .say(
                                &ctx.http,
                                format!("Member {} assigned successsfully", &user_id),
                            )
                            .await
                            .unwrap(),
                        Err(_) => msg
                            .channel_id
                            .say(&ctx.http, "Failed to assign member.")
                            .await
                            .unwrap(),
                    };
                }
                Err(diesel::NotFound) => {
                    msg.channel_id
                        .say(&ctx.http, "Team not found!")
                        .await
                        .unwrap();
                }
                Err(e) => {
                    println!("Database error: {:?}", e);
                    msg.channel_id
                        .say(&ctx.http, "An error occurred while fetching team.")
                        .await
                        .unwrap();
                }
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
