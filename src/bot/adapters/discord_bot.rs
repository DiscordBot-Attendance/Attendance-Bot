use crate::config::database::establish_connection;
use crate::config::database::DBPool;
use serenity::all::Message;
use serenity::all::Ready;
use serenity::{async_trait, prelude::*};

use crate::bot::adapters::handler::{attendance, auth, members, teams};

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

        // Show help message
        if msg.content.starts_with("!AB help") {
            if let Err(e) = msg.channel_id.say(&ctx.http, HELP_MESSAGES).await {
                println!("Error sending message: {e:?}");
            }
            return;
        }

        // Get database connection
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

        // Handle commands
        if msg.content.starts_with("!AB check_in") {
            attendance::handle_check_in(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB check_out") {
            attendance::handle_check_out(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB register") {
            auth::handle_register(&ctx, &msg, &mut db_conn).await;
        } else if msg.content.starts_with("!AB login") {
            auth::handle_login(&ctx, &msg, &mut db_conn).await;
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

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

//impl Handler {
//    async fn handle_show_team(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        let admin_discord_id = msg.author.id.to_string();
//
//        let teams = match show_team(db_conn, &admin_discord_id[..]) {
//            Ok(teams) => teams,
//            Err(e) => {
//                self.send_message(&ctx, &msg.channel_id, &format!("Error: {}", e))
//                    .await;
//                return;
//            }
//        };
//
//        // Check if the admin has any teams
//        if teams.is_empty() {
//            self.send_message(&ctx, &msg.channel_id, "You have no teams.")
//                .await;
//            return;
//        }
//
//        // Create a table from the teams
//        let mut table = Table::new(teams);
//
//        // Send the table as a message
//        self.send_message(
//            &ctx,
//            &msg.channel_id,
//            &format!("Your teams:\n```\n{}\n```", table.with(Style::rounded())),
//        )
//        .await;
//    }
//
//    async fn handle_check_in(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        use crate::schema::members::dsl::{discord_id, id as members_id, members};
//        use crate::schema::teams::dsl::{id as teams_id, name, teams};
//
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 3 {
//            self.send_message(
//                &ctx,
//                &msg.channel_id,
//                "Usage: !AB check-in <team-name> <status>",
//            )
//            .await;
//            return;
//        }
//
//        let team_name = args[2];
//        let status = args[3];
//
//        let user_id_str = msg.author.id.to_string();
//        let user_id = members
//            .filter(discord_id.eq(&user_id_str))
//            .select(members_id)
//            .first::<i32>(db_conn)
//            .ok();
//
//        let team_id = match teams
//            .filter(name.eq(team_name))
//            .select(teams_id)
//            .first::<i32>(db_conn)
//        {
//            Ok(id) => id,
//            Err(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Team not found.")
//                    .await;
//                return;
//            }
//        };
//
//        match attendance_service::check_in(
//            db_conn,
//            user_id.expect("no user id"),
//            team_id,
//            status.to_string(),
//        ) {
//            Ok(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Checked in successfully!")
//                    .await;
//            }
//            Err(e) => {
//                self.send_message(&ctx, &msg.channel_id, &format!("Failed to check in! {}", e))
//                    .await;
//            }
//        }
//    }
//
//    async fn handle_check_out(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        use crate::schema::members::dsl::{discord_id, id as members_id, members};
//
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 3 {
//            self.send_message(&ctx, &msg.channel_id, "Usage: !AB check-out <team-name>")
//                .await;
//            return;
//        }
//
//        // Parse the user ID from the message author
//        let user_id_str = msg.author.id.to_string();
//        let user_id = match members
//            .filter(discord_id.eq(&user_id_str))
//            .select(members_id)
//            .first::<i32>(db_conn)
//        {
//            Ok(id) => id,
//            Err(_) => {
//                self.send_message(&ctx, &msg.channel_id, "User not found!")
//                    .await;
//                return;
//            }
//        };
//
//        // Call the check-out service
//        match attendance_service::check_out(db_conn, user_id) {
//            Ok(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Checked out successfully!")
//                    .await;
//            }
//            Err(e) => {
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    &format!("Failed to check out: {}", e),
//                )
//                .await;
//            }
//        }
//    }
//
//    async fn handle_register(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 3 {
//            self.send_message(&ctx, &msg.channel_id, "Usage: !AB register <password>")
//                .await;
//            return;
//        }
//
//        let password = args[2];
//
//        match user_repository::register_admin(
//            db_conn,
//            &msg.author.id.to_string(),
//            &msg.author.name,
//            password,
//        ) {
//            Ok(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Admin registered successfully!")
//                    .await;
//            }
//            Err(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Registration failed!")
//                    .await;
//            }
//        }
//    }
//
//    async fn handle_login(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 3 {
//            self.send_message(&ctx, &msg.channel_id, "Usage: !AB login <password>")
//                .await;
//            return;
//        }
//
//        let password = args[2];
//
//        match user_repository::authentication_admin(db_conn, &msg.author.id.to_string(), password) {
//            Ok(true) => {
//                let token = auth_service::generate_token(&msg.author.id.to_string());
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    &format!("Login successful! Your token: {}", token),
//                )
//                .await;
//            }
//            _ => {
//                self.send_message(&ctx, &msg.channel_id, "Invalid credentials!")
//                    .await;
//            }
//        }
//    }
//
//    async fn handle_create_team(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 3 {
//            self.send_message(&ctx, &msg.channel_id, "Usage: !AB create_team <team_name>")
//                .await;
//            return;
//        }
//
//        let team_name = args[2];
//
//        use crate::schema::users::dsl::*;
//        let dc_user_id = msg.author.id.to_string();
//        let admin = match users
//            .filter(discord_id.eq(dc_user_id))
//            .first::<User>(db_conn)
//        {
//            Ok(admin) => admin,
//            Err(diesel::NotFound) => {
//                self.send_message(&ctx, &msg.channel_id, "User not found!")
//                    .await;
//                return;
//            }
//            Err(e) => {
//                println!("Database error: {:?}", e);
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    "An error occurred while fetching user.",
//                )
//                .await;
//                return;
//            }
//        };
//
//        match team_service::register_team(db_conn, team_name, admin.id) {
//            Ok(_) => {
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    &format!("Team '{}' registered successfully!", team_name),
//                )
//                .await;
//            }
//            Err(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Failed to register team.")
//                    .await;
//            }
//        }
//    }
//
//    async fn handle_add_member(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 6 {
//            self.send_message(
//                &ctx,
//                &msg.channel_id,
//                "Usage: !AB add-member <team_name> @user as User_full_name",
//            )
//            .await;
//            return;
//        }
//
//        let user_id = args[3]
//            .replace("<@!", "")
//            .replace("<@", "")
//            .replace(">", "");
//
//        let username = args[5].to_string();
//
//        use crate::schema::teams::dsl::*;
//        let team_name = args[2];
//        let team_id = match teams
//            .filter(name.eq(team_name))
//            .select(id)
//            .first::<i32>(db_conn)
//        {
//            Ok(ids) => ids,
//            Err(diesel::NotFound) => {
//                self.send_message(&ctx, &msg.channel_id, "Team not found!")
//                    .await;
//                return;
//            }
//            Err(e) => {
//                println!("Database error: {:?}", e);
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    "An error occurred while fetching team.",
//                )
//                .await;
//                return;
//            }
//        };
//
//        match team_service::add_member(db_conn, &user_id, username, team_id) {
//            Ok(_) => {
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    &format!("Member {} assigned successfully", &user_id),
//                )
//                .await;
//            }
//            Err(_) => {
//                self.send_message(&ctx, &msg.channel_id, "Failed to assign member.")
//                    .await;
//            }
//        }
//    }
//
//    async fn handle_show_members(&self, ctx: &Context, msg: &Message, db_conn: &mut PgConnection) {
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 3 {
//            self.send_message(&ctx, &msg.channel_id, "Usage: !AB show_members <team_name>")
//                .await;
//            return;
//        }
//
//        let team_name = args[2];
//
//        // Fetch members from the database
//        let members = match get_members_by_team(db_conn, team_name) {
//            Ok(members) => members,
//            Err(e) => {
//                eprintln!("❌ Error getting members for '{}': {}", team_name, e);
//                self.send_message(
//                    &ctx,
//                    &msg.channel_id,
//                    &format!("Error getting members: {}", e),
//                )
//                .await;
//                return;
//            }
//        };
//
//        // Check if there are any members
//        if members.is_empty() {
//            self.send_message(&ctx, &msg.channel_id, "No members found.")
//                .await;
//            return;
//        }
//
//        // Create a table from the members
//        let table = Table::new(members)
//            .with(Style::rounded()) // Apply rounded style to the table
//            .to_string();
//
//        // Send the table as a message
//        self.send_message(
//            &ctx,
//            &msg.channel_id,
//            &format!("Team members:\n```\n{}\n```", table),
//        )
//        .await;
//    }
//
//    async fn handle_show_member_attendance(
//        &self,
//        ctx: &Context,
//        msg: &Message,
//        db_conn: &mut PgConnection,
//    ) {
//        let args: Vec<&str> = msg.content.split_whitespace().collect();
//        if args.len() < 2 {
//            self.send_message(
//                &ctx,
//                &msg.channel_id,
//                "Usage: !AB show_member_attendance <team_name>",
//            )
//            .await;
//            return;
//        }
//
//        let team_name = args[2];
//
//        // Fetch attendance data from the database
//        let attendance_data = match get_member_attendance(db_conn, team_name) {
//            Ok(data) => data,
//            Err(e) => {
//                self.send_message(&ctx, &msg.channel_id, &format!("Error: {}", e))
//                    .await;
//                return;
//            }
//        };
//
//        // Check if there are any attendance records
//        if attendance_data.is_empty() {
//            self.send_message(
//                &ctx,
//                &msg.channel_id,
//                "No attendance records found for this team.",
//            )
//            .await;
//            return;
//        }
//
//        // Create a table from the attendance data
//        let table = Table::new(attendance_data)
//            .with(Style::rounded()) // Apply rounded style to the table
//            .to_string();
//
//        // Send the table as a message
//        self.send_message(
//            &ctx,
//            &msg.channel_id,
//            &format!("Attendance for team '{}':\n```\n{}\n```", team_name, table),
//        )
//        .await;
//    }
//
//    async fn send_message(&self, ctx: &Context, channel_id: &ChannelId, message: &str) {
//        if let Err(e) = channel_id.say(&ctx.http, message).await {
//            println!("Error sending message: {e:?}");
//        }
//    }
//}
