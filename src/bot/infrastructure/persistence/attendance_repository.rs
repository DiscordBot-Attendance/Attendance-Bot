use crate::{
    bot::domain::{
        model::{CheckInAttendance, Member, MemberAttendance},
        table::MemberAttendanceTable,
    },
    schema::{
        members::id,
        teams::dsl::{id as team_ids, teams},
    },
};
use chrono::Utc;
use diesel::dsl::exists;
use diesel::prelude::*;

/// Records a check-in for a member in a specific team.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `mem_id` - The ID of the member checking in.
/// * `team_id` - The ID of the team the member is checking into.
/// * `status` - The status of the check-in (e.g., "Present", "Late").
///
/// # Errors
/// Returns an error if:
/// - The member does not exist.
/// - The team does not exist.
/// - The check-in record cannot be inserted into the database.
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

/// Records a check-out for a member.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `user_id` - The ID of the member checking out.
///
/// # Errors
/// Returns an error if:
/// - No active check-in is found for the member.
/// - The check-out record cannot be updated in the database.
pub fn check_out(conn: &mut PgConnection, user_id: i32) -> Result<(), String> {
    use crate::schema::member_attendance::dsl::*;

    let now = Utc::now().naive_utc();

    let check_out_member = diesel::update(
        member_attendance
        .filter(member_id.eq(user_id))
        .filter(check_out_time.is_null()),
    )
        .set(check_out_time.eq(now))
        .execute(conn)
        .map_err(|e| format!("Failed to check out: {}", e))?;

    if check_out_member == 0 {
        return Err("No active check-in found!".to_string());
    }

    Ok(())
}

/// Retrieves attendance records for all members of a specific team.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `team_name` - The name of the team whose attendance records are being fetched.
///
/// # Returns
/// Returns a `Vec<MemberAttendanceTable>` containing the attendance records for the team.
///
/// # Errors
/// Returns an error if:
/// - The team does not exist.
/// - The attendance records cannot be fetched from the database.
pub fn get_member_attendance_by_team(
    conn: &mut PgConnection,
    team_name: &str,
) -> Result<Vec<MemberAttendanceTable>, String> {
    use crate::schema::{member_attendance, members, teams};

    // Find the team ID by name
    let team_id: i32 = teams::table
        .filter(teams::name.eq(team_name))
        .select(teams::id)
        .first::<i32>(conn)
        .map_err(|e| {
            if e == diesel::result::Error::NotFound {
                format!("Team '{}' not found", team_name)
            } else {
                format!("Failed to fetch team '{}': {}", team_name, e)
            }
        })?;

    // Fetch attendance records for the team
    let attendance_data: Vec<(MemberAttendance, Member)> = member_attendance::table
        .inner_join(
            members::table.on(members::id
                .nullable()
                .eq(member_attendance::member_id.nullable())),
        )
        .filter(member_attendance::team_id.eq(team_id))
        .load::<(MemberAttendance, Member)>(conn)
        .map_err(|e| format!("Failed to fetch attendance for team '{}': {}", team_name, e))?;

    // Map the data to the MemberAttendanceTable struct
    let attendance_tables = attendance_data
        .into_iter()
        .map(|(attendance, member)| MemberAttendanceTable {
            username: member.username,
            check_in_time: attendance
                .check_in_time
                .map(|time| time.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "N/A".to_string()),
                check_out_time: attendance
                    .check_out_time
                    .map(|time| time.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "N/A".to_string()),
                    status: attendance.status.unwrap_or_else(|| "N/A".to_string()),
        })
    .collect();

    Ok(attendance_tables)
}
