use crate::bot::{
    domain::table::MemberAttendanceTable, infrastructure::persistence::attendance_repository,
};
use diesel::PgConnection;

pub fn check_in(
    conn: &mut PgConnection,
    member_id: i32,
    team_id: i32,
    status: String,
) -> Result<(), String> {
    attendance_repository::check_in(conn, member_id, team_id, status)
}

pub fn check_out(conn: &mut PgConnection, member_id: i32) -> Result<(), String> {
    attendance_repository::check_out(conn, member_id)
}

pub fn get_member_attendance(
    conn: &mut PgConnection,
    team_name: &str,
) -> Result<Vec<MemberAttendanceTable>, String> {
    attendance_repository::get_member_attendance_by_team(conn, team_name)
}
