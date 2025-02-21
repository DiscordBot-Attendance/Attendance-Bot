use crate::bot::{
    domain::table::MemberAttendanceTable, infrastructure::persistence::attendance_repository,
};
use diesel::PgConnection;

/// Records a check-in for a member in a specific team.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `member_id` - The ID of the member checking in.
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
    member_id: i32,
    team_id: i32,
    status: String,
) -> Result<(), String> {
    attendance_repository::check_in(conn, member_id, team_id, status)
}

/// Records a check-out for a member.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `member_id` - The ID of the member checking out.
///
/// # Errors
/// Returns an error if:
/// - No active check-in is found for the member.
/// - The check-out record cannot be updated in the database.
pub fn check_out(conn: &mut PgConnection, member_id: i32) -> Result<(), String> {
    attendance_repository::check_out(conn, member_id)
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
pub fn get_member_attendance(
    conn: &mut PgConnection,
    team_name: &str,
) -> Result<Vec<MemberAttendanceTable>, String> {
    attendance_repository::get_member_attendance_by_team(conn, team_name)
}
