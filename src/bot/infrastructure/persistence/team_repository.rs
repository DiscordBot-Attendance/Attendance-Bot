use crate::bot::domain::model::{NewMember, NewTeam};
use chrono::Utc;
use diesel::prelude::*;

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
