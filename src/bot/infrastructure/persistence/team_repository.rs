use crate::bot::domain::model::{NewMember, NewTeam, Team};
use crate::bot::domain::table::TeamTable;
use chrono::Utc;
use diesel::prelude::*;

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

pub fn assign_member(
    conn: &mut PgConnection,
    dc_id: &str,
    team_id_value: i32,
) -> Result<(), String> {
    use crate::schema::members::dsl::*;

    let new_member = NewMember {
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
