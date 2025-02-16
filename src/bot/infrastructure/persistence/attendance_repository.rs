use crate::{
    bot::domain::model::{CheckInAttendance, CheckOutAttendance},
    schema::{
        member_attendance::member_id,
        members::id,
        teams::dsl::{id as team_ids, teams},
    },
};
use chrono::Utc;
use diesel::dsl::exists;
use diesel::prelude::*;

pub fn check_in(
    conn: &mut PgConnection,
    mem_id: i32,
    team_id: i32,
    status: String,
) -> Result<(), String> {
    use crate::schema::{member_attendance::dsl::member_attendance, members::dsl::members};

    // Check if the member_id exists in the members table
    let member_exists: bool = diesel::select(exists(
        members.filter(id.eq(mem_id)), // id should be the primary key of members
    ))
    .get_result(conn)
    .map_err(|e| format!("Failed to check member existence: {}", e))?;

    if !member_exists {
        return Err(format!("Member with ID {} does not exist.", mem_id));
    }

    // Check if the team_id exists in the members table
    let team_exists: bool = diesel::select(exists(
        teams.filter(team_ids.eq(team_id)), // id should be the primary key of members
    ))
    .get_result(conn)
    .map_err(|e| format!("Failed to check team existence: {}", e))?;

    if !team_exists {
        return Err(format!("Member with ID {} does not exist.", team_id));
    }

    // Insert into member_attendance
    let check_in_member = CheckInAttendance {
        member_id: mem_id,
        team_id,
        check_in_time: Utc::now().naive_utc(),
        date: Utc::now().date_naive(),
        status,
    };

    diesel::insert_into(member_attendance)
        .values(check_in_member)
        .execute(conn)
        .map_err(|e| format!("Failed to insert into member_attendance: {}", e))?;

    Ok(())
}

pub fn check_out(conn: &mut PgConnection, user_id: i32) -> Result<(), String> {
    use crate::schema::member_attendance::dsl::*;

    let now = Utc::now().naive_utc();

    let check_out_member = diesel::update(
        member_attendance.filter(
            member_id
                .eq::<Option<i32>>(None)
                .and(check_out_time.is_null()),
        ),
    )
    .set(check_out_time.eq(now))
    .execute(conn)
    .map_err(|_| "Failed to check out!")?;

    if check_out_member == 0 {
        return Err("No active check-in found!".to_string());
    }

    Ok(())
}
