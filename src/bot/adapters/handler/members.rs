use diesel::PgConnection;
use serenity::client::Context;
use serenity::model::channel::Message;
use tabled::{settings::Style, Table};

use crate::bot::application::services::team_service::get_members_by_team;

/// Handles displaying the members of a specific team.
///
/// # Arguments
/// * `ctx` - The context of the event.
/// * `msg` - The message that triggered the event.
/// * `db_conn` - A mutable reference to the PostgreSQL connection.
///
/// # Behavior
/// - Parses the command arguments to extract the team name.
/// - Fetches the members of the specified team from the database.
/// - Displays the members in a formatted table.
/// - Sends the table as a message back to the user.
pub async fn handle_show_members(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 3 {
        send_message(ctx, &msg.channel_id, "Usage: !AB show_members <team_name>").await;
        return;
    }

    let team_name = args[2];

    // Fetch members from the database
    let members = match get_members_by_team(db_conn, team_name) {
        Ok(members) => members,
        Err(e) => {
            send_message(ctx, &msg.channel_id, &format!("Error: {}", e)).await;
            return;
        }
    };

    // Check if there are any members
    if members.is_empty() {
        send_message(ctx, &msg.channel_id, "No members found.").await;
        return;
    }

    // Create a table from the members
    let table = Table::new(members).with(Style::rounded()).to_string();

    // Send the table as a message
    send_message(
        ctx,
        &msg.channel_id,
        &format!("Team members:\n```\n{}\n```", table),
    )
    .await;
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
