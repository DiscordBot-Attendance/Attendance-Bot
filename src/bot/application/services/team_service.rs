use crate::bot::{
    domain::table::{MemberTable, TeamTable},
    infrastructure::persistence::team_repository::{self},
};
use diesel::PgConnection;

/// Registers a new team in the database.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `name` - The name of the team to register.
/// * `admin_id` - The ID of the admin creating the team.
///
/// # Errors
/// Returns an error if the team cannot be created in the database.
pub fn register_team(conn: &mut PgConnection, name: &str, admin_id: i32) -> Result<(), String> {
    team_repository::create_team(conn, name, admin_id)
}

/// Adds a new member to a team in the database.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `discord_id` - The Discord ID of the member.
/// * `username` - The username of the member.
/// * `team_id` - The ID of the team to which the member is being added.
///
/// # Errors
/// Returns an error if the member cannot be added to the team.
pub fn add_member(
    conn: &mut PgConnection,
    discord_id: &str,
    username: String,
    team_id: i32,
) -> Result<(), String> {
    team_repository::assign_member(conn, discord_id, username, team_id)
}

/// Retrieves all teams created by a specific admin.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `admin_discord_id` - The Discord ID of the admin whose teams are being fetched.
///
/// # Returns
/// Returns a `Vec<TeamTable>` containing the teams created by the admin.
///
/// # Errors
/// Returns an error if the teams cannot be fetched from the database.
pub fn show_team(
    conn: &mut PgConnection,
    admin_discord_id: &str,
) -> Result<Vec<TeamTable>, String> {
    team_repository::get_admin_teams(conn, admin_discord_id)
}

/// Retrieves all members of a specific team.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `team_name` - The name of the team whose members are being fetched.
///
/// # Returns
/// Returns a `Vec<MemberTable>` containing the members of the team.
///
/// # Errors
/// Returns an error if the members cannot be fetched from the database.
pub fn get_members_by_team(
    conn: &mut PgConnection,
    team_name: &str,
) -> Result<Vec<MemberTable>, String> {
    team_repository::get_members(conn, team_name)
}
