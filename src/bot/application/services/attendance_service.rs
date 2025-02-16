use crate::bot::infrastructure::persistence::attendance_repository;
use diesel::PgConnection;

pub fn check_in(conn: &mut PgConnection, member_id: i32, team_id: i32, status: String) -> Result<(), String> {
    attendance_repository::check_in(conn, member_id, team_id, status)
}

pub fn check_out(conn: &mut PgConnection, member_id: i32) -> Result<(), String> {
    attendance_repository::check_out(conn, member_id)
}
