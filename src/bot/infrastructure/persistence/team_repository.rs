use crate::bot::domain::model::{Member, NewMember, NewTeam, Team};
use crate::bot::domain::table::{MemberTable, TeamTable};
use chrono::Utc;
use diesel::prelude::*;

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
/// Returns an error if:
/// - The admin user cannot be found.
/// - The teams cannot be fetched from the database.
pub fn get_admin_teams(
    conn: &mut PgConnection,
    admin_discord_id: &str,
) -> Result<Vec<TeamTable>, String> {
    use crate::schema::teams::dsl::*;
    use crate::schema::users::dsl::{discord_id, id as user_id, users};

    // Find the admin's user ID
    let admin_user_id: i32 = users
        .filter(discord_id.eq(admin_discord_id))
        .select(user_id)
        .first(conn)
        .map_err(|e| format!("Failed to find admin: {}", e))?;

    // Fetch teams created by the admin
    let teams_data: Vec<Team> = teams
        .filter(admin_id.eq(admin_user_id))
        .load::<Team>(conn)
        .map_err(|e| format!("Failed to fetch teams: {}", e))?;

    // Map the teams to the TeamTable struct
    let team_tables = teams_data
        .into_iter()
        .map(|team| TeamTable {
            name: team.name,
            created_at: team
                .created_at
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "N/A".to_string()),
        })
        .collect();

    Ok(team_tables)
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
/// Returns an error if:
/// - The team cannot be found.
/// - The members cannot be fetched from the database.
pub fn get_members(conn: &mut PgConnection, team_name: &str) -> Result<Vec<MemberTable>, String> {
    use crate::schema::members::dsl::*;
    use crate::schema::teams::dsl::{id as team_id_column, name as team_name_column, teams};

    // Find the team ID by name
    let team_ids: i32 = teams
        .filter(team_name_column.eq(team_name))
        .select(team_id_column)
        .first(conn)
        .map_err(|e| format!("Failed to find team: {}", e))?;

    // Fetch members of the team
    let members_data: Vec<Member> = members
        .filter(team_id.eq(team_ids))
        .load::<Member>(conn)
        .map_err(|e| format!("Failed to fetch members: {}", e))?;

    // Map the members to the MemberTable struct
    let member_tables = members_data
        .into_iter()
        .map(|member| MemberTable {
            username: member.username,
            join_date: member
                .join_date
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|| "N/A".to_string()),
        })
        .collect();

    Ok(member_tables)
}

/// Creates a new team in the database.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `name` - The name of the team to create.
/// * `admin` - The ID of the admin creating the team.
///
/// # Errors
/// Returns an error if the team cannot be created in the database.
pub fn create_team(conn: &mut PgConnection, name: &str, admin: i32) -> Result<(), String> {
    use crate::schema::teams::dsl::teams;

    let new_team = NewTeam {
        name: name.to_string(),
        admin_id: admin,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(teams)
        .values(&new_team)
        .execute(conn)
        .map_err(|_| "Failed to create team")?;

    Ok(())
}

/// Assigns a member to a team.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `dc_id` - The Discord ID of the member.
/// * `username_string` - The username of the member.
/// * `team_id_value` - The ID of the team to which the member is being assigned.
///
/// # Errors
/// Returns an error if the member cannot be assigned to the team.
pub fn assign_member(
    conn: &mut PgConnection,
    dc_id: &str,
    username_string: String,
    team_id_value: i32,
) -> Result<(), String> {
    use crate::schema::members::dsl::*;

    let new_member = NewMember {
        username: username_string,
        discord_id: dc_id,
        team_id: team_id_value,
        join_date: Some(Utc::now().naive_utc().into()),
        position: "Default".to_string(),
    };

    diesel::insert_into(members)
        .values(&new_member)
        .execute(conn)
        .map_err(|_| "Failed to assign member")?;

    Ok(())
}
