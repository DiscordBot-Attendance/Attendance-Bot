use crate::bot::{
    domain::table::{MemberTable, TeamTable},
    infrastructure::persistence::team_repository::{self},
};
use diesel::PgConnection;

pub fn register_team(conn: &mut PgConnection, name: &str, admin_id: i32) -> Result<(), String> {
    team_repository::create_team(conn, name, admin_id)
}

pub fn add_member(
    conn: &mut PgConnection,
    discord_id: &str,
    username: String,
    team_id: i32,
) -> Result<(), String> {
    team_repository::assign_member(conn, discord_id, username, team_id)
}

pub fn show_team(
    conn: &mut PgConnection,
    admin_discord_id: &str,
) -> Result<Vec<TeamTable>, String> {
    team_repository::get_admin_teams(conn, admin_discord_id)
}

pub fn get_members_by_team(
    conn: &mut PgConnection,
    team_name: &str,
) -> Result<Vec<MemberTable>, String> {
    team_repository::get_members(conn, team_name)
}
