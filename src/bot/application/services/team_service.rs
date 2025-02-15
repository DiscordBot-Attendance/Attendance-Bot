use crate::bot::infrastructure::persistence::team_repository;
use diesel::PgConnection;

pub fn register_team(conn: &mut PgConnection, name: &str, admin_id: i32) -> Result<(), String> {
    team_repository::create_team(conn, name, admin_id)
}

pub fn add_member(conn: &mut PgConnection, discord_id: &str, team_id: i32) -> Result<(), String> {
    team_repository::assign_member(conn, discord_id, team_id)
}
