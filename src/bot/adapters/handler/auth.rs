use diesel::PgConnection;
use serenity::client::Context;
use serenity::model::channel::Message;

use crate::bot::{
    application::services::auth_service,
    infrastructure::persistence::user_repository::{authentication_admin, register_admin},
};

/// Handles the registration of an admin user.
///
/// # Arguments
/// * `ctx` - The context of the event.
/// * `msg` - The message that triggered the event.
/// * `db_conn` - A mutable reference to the PostgreSQL connection.
///
/// # Behavior
/// - Parses the command arguments to extract the password.
/// - Registers the admin user in the database.
/// - Sends a success or error message back to the user.
pub async fn handle_register(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 3 {
        send_message(ctx, &msg.channel_id, "Usage: !AB register <password>").await;
        return;
    }

    let password = args[2];

    match register_admin(
        db_conn,
        &msg.author.id.to_string(),
        &msg.author.name,
        password,
    ) {
        Ok(_) => {
            send_message(ctx, &msg.channel_id, "Admin registered successfully!").await;
        }
        Err(_) => {
            send_message(ctx, &msg.channel_id, "Registration failed!").await;
        }
    }
}

/// Handles the login of an admin user.
///
/// # Arguments
/// * `ctx` - The context of the event.
/// * `msg` - The message that triggered the event.
/// * `db_conn` - A mutable reference to the PostgreSQL connection.
///
/// # Behavior
/// - Parses the command arguments to extract the password.
/// - Authenticates the admin user using the provided credentials.
/// - Generates and sends a JWT token if authentication is successful.
/// - Sends an error message if authentication fails.
pub async fn handle_login(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 3 {
        send_message(ctx, &msg.channel_id, "Usage: !AB login <password>").await;
        return;
    }

    let password = args[2];

    match authentication_admin(db_conn, &msg.author.id.to_string(), password) {
        Ok(true) => {
            let token = auth_service::generate_token(&msg.author.id.to_string());
            send_message(
                ctx,
                &msg.channel_id,
                &format!("Login successful! Your token: {}", token),
            )
            .await;
        }
        _ => {
            send_message(ctx, &msg.channel_id, "Invalid credentials!").await;
        }
    }
}

/// Sends a message to a specific channel.
///
/// # Arguments
/// * `ctx` - The context of the event.
/// * `channel_id` - The ID of the channel to send the message to.
/// * `message` - The message to send.
///
/// # Behavior
/// - Attempts to send the message to the specified channel.
/// - Logs an error if the message fails to send.
async fn send_message(ctx: &Context, channel_id: &serenity::model::id::ChannelId, message: &str) {
    if let Err(e) = channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {e:?}");
    }
}
