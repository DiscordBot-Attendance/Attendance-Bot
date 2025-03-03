use crate::config::database::establish_connection;
use crate::config::database::DBPool;
use serenity::all::Message;
use serenity::all::Ready;
use serenity::{async_trait, prelude::*};

use crate::bot::adapters::handler::{attendance, auth, members, teams};

use crate::config::constant::HELP_MESSAGES;

/// Represents the event handler for the bot.
///
/// This struct is responsible for handling events such as messages and connection readiness.
pub struct Handler {
    db_pool: DBPool,
}

impl Handler {
    /// Creates a new instance of the `Handler`.
    ///
    /// # Arguments
    /// * `db_pool` - A connection pool to the database.
    pub fn new(db_pool: DBPool) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl EventHandler for Handler {
    /// Handles incoming messages and processes commands.
    ///
    /// # Arguments
    /// * `ctx` - The context of the event.
    /// * `msg` - The message that triggered the event.
    async fn message(&self, ctx: Context, msg: Message) {
        // Establish a database connection.
        let conn = establish_connection();

        // Show help message if the command is "!AB help".
        if msg.content.starts_with("!AB help") {
            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGES).await {
                println!("Error sending message: {e:?}");
            }
            return;
        }

        // Get a database connection from the pool.
        let mut db_conn = match conn.get() {
            Ok(conn) => conn,
            Err(_) => {
                if let Err(e) = msg
                    .channel_id
                    .say(&ctx.http, "Failed to get DB connection")
                    .await
                {
                    println!("Error sending message: {e:?}");
                }
                return;
            }
        };

        // Handle specific commands based on the message content.
        if msg.content.starts_with("!AB check_in") {
            attendance::handle_check_in(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB check_out") {
            attendance::handle_check_out(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB register") {
            auth::handle_register(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB create_team") {
            teams::handle_create_team(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB add_member") {
            teams::handle_add_member(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB show_members") {
            members::handle_show_members(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB show_members_attendance") {
            attendance::handle_show_member_attendance(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB show_team") {
            teams::handle_show_team(&ctx, &msg, &mut db_conn).await;
        }
    }

    /// Handles the bot's readiness event.
    ///
    /// # Arguments
    /// * `_` - The context of the event (unused in this implementation).
    /// * `ready` - The readiness event data.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
