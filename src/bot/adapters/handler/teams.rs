use diesel::query_dsl::methods::{FilterDsl, SelectDsl};
use diesel::{ExpressionMethods as _, PgConnection, RunQueryDsl};
use serenity::client::Context;
use serenity::model::channel::Message;
use tabled::{settings::Style, Table};

use crate::bot::application::services::team_service::{self, show_team};
use crate::bot::domain::model::User;

pub async fn handle_create_team(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 3 {
        send_message(ctx, &msg.channel_id, "Usage: !AB create_team <team_name>").await;
        return;
    }

    let team_name = args[2];

    use crate::schema::users::dsl::*;
    let dc_user_id = msg.author.id.to_string();
    let admin = match users
        .filter(discord_id.eq(dc_user_id))
        .first::<User>(db_conn)
    {
        Ok(admin) => admin,
        Err(diesel::NotFound) => {
            send_message(ctx, &msg.channel_id, "User not found!").await;
            return;
        }
        Err(e) => {
            println!("Database error: {:?}", e);
            send_message(
                ctx,
                &msg.channel_id,
                "An error occurred while fetching user.",
            )
            .await;
            return;
        }
    };

    match team_service::register_team(db_conn, team_name, admin.id) {
        Ok(_) => {
            send_message(
                ctx,
                &msg.channel_id,
                &format!("Team '{}' registered successfully!", team_name),
            )
            .await;
        }
        Err(_) => {
            send_message(ctx, &msg.channel_id, "Failed to register team.").await;
        }
    }
}

pub async fn handle_add_member(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    let args: Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 6 {
        send_message(
            ctx,
            &msg.channel_id,
            "Usage: !AB add_member <team_name> @user as User_full_name",
        )
        .await;
        return;
    }

    let user_id = args[3]
        .replace("<@!", "")
        .replace("<@", "")
        .replace(">", "");

    let username = args[5].to_string();

    use crate::schema::teams::dsl::*;
    let team_name = args[2];
    let team_id = match teams
        .filter(name.eq(team_name))
        .select(id)
        .first::<i32>(db_conn)
    {
        Ok(ids) => ids,
        Err(diesel::NotFound) => {
            send_message(ctx, &msg.channel_id, "Team not found!").await;
            return;
        }
        Err(e) => {
            println!("Database error: {:?}", e);
            send_message(
                ctx,
                &msg.channel_id,
                "An error occurred while fetching team.",
            )
            .await;
            return;
        }
    };

    match team_service::add_member(db_conn, &user_id, username, team_id) {
        Ok(_) => {
            send_message(
                ctx,
                &msg.channel_id,
                &format!("Member {} assigned successfully", &user_id),
            )
            .await;
        }
        Err(_) => {
            send_message(ctx, &msg.channel_id, "Failed to assign member.").await;
        }
    }
}

pub async fn handle_show_team(ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
    let admin_discord_id = msg.author.id.to_string();

    let teams = match show_team(db_conn, &admin_discord_id) {
        Ok(teams) => teams,
        Err(e) => {
            send_message(ctx, &msg.channel_id, &format!("Error: {}", e)).await;
            return;
        }
    };

    // Check if the admin has any teams
    if teams.is_empty() {
        send_message(ctx, &msg.channel_id, "You have no teams.").await;
        return;
    }

    // Create a table from the teams
    let table = Table::new(teams).with(Style::rounded()).to_string();

    // Send the table as a message
    send_message(
        ctx,
        &msg.channel_id,
        &format!("Your teams:\n```\n{}\n```", table),
    )
    .await;
}

async fn send_message(ctx: &Context, channel_id: &serenity::model::id::ChannelId, message: &str) {
    if let Err(e) = channel_id.say(&ctx.http, message).await {
        println!("Error sending message: {e:?}");
    }
}
