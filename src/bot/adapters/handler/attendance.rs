use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serenity::client::Context;
use serenity::model::channel::Message;
use tabled::{settings::Style, Table};

use crate::bot::application::services::attendance_service::{self, get_member_attendance};

pub async fn handle_check_in(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    use crate::schema::members::dsl::{discord_id, id as members_id, members};
    use crate::schema::teams::dsl::{id as teams_id, name, teams};

    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 4 {
        send_message(
            ctx,
            &msg.channel_id,
            "Usage: !AB check-in <team-name> <status>",
        )
        .await;
        return;
    }

    let team_name = args[2];
    let status = args[3];

    let user_id_str = msg.author.id.to_string();
    let user_id = members
        .filter(discord_id.eq(&user_id_str))
        .select(members_id)
        .first::<i32>(db_conn)
        .ok();

    let team_id = match teams
        .filter(name.eq(team_name))
        .select(teams_id)
        .first::<i32>(db_conn)
    {
        Ok(id) => id,
        Err(_) => {
            send_message(ctx, &msg.channel_id, "Team not found.").await;
            return;
        }
    };

    match attendance_service::check_in(
        db_conn,
        user_id.expect("no user id"),
        team_id,
        status.to_string(),
    ) {
        Ok(_) => {
            send_message(ctx, &msg.channel_id, "Checked in successfully!").await;
        }
        Err(e) => {
            send_message(ctx, &msg.channel_id, &format!("Failed to check in! {}", e)).await;
        }
    }
}

pub async fn handle_check_out(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    use crate::schema::members::dsl::{discord_id, id as members_id, members};

    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 3 {
        send_message(ctx, &msg.channel_id, "Usage: !AB check-out <team-name>").await;
        return;
    }

    // Parse the user ID from the message author
    let user_id_str = msg.author.id.to_string();
    let user_id = match members
        .filter(discord_id.eq(&user_id_str))
        .select(members_id)
        .first::<i32>(db_conn)
    {
        Ok(id) => id,
        Err(_) => {
            send_message(ctx, &msg.channel_id, "User not found!").await;
            return;
        }
    };

    // Call the check-out service
    match attendance_service::check_out(db_conn, user_id) {
        Ok(_) => {
            send_message(ctx, &msg.channel_id, "Checked out successfully!").await;
        }
        Err(e) => {
            send_message(ctx, &msg.channel_id, &format!("Failed to check out: {}", e)).await;
        }
    }
}

pub async fn handle_show_member_attendance(
    ctx: &Context,
    msg: &Message,
    db_conn: &mut PgConnection,
) {
    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 2 {
        send_message(
            ctx,
            &msg.channel_id,
            "Usage: !AB show_members_attendance <team_name>",
        )
        .await;
        return;
    }

    let team_name = args[2];

    // Fetch attendance data from the database
    let attendance_data = match get_member_attendance(db_conn, team_name) {
        Ok(data) => data,
        Err(e) => {
            send_message(ctx, &msg.channel_id, &format!("Error: {}", e)).await;
            return;
        }
    };

    // Check if there are any attendance records
    if attendance_data.is_empty() {
        send_message(
            ctx,
            &msg.channel_id,
            "No attendance records found for this team.",
        )
        .await;
        return;
    }

    // Create a table from the attendance data
    let table = Table::new(attendance_data)
        .with(Style::rounded())
        .to_string();

    // Send the table as a message
    send_message(
        ctx,
        &msg.channel_id,
        &format!("Attendance for team '{}':\n```\n{}\n```", team_name, table),
    )
    .await;
}

async fn send_message(ctx: &Context, channel_id: &serenity::model::id::ChannelId, message: &str) {
    if let Err(e) = channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {e:?}");
    }
}
