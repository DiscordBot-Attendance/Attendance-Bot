use actix_web::{web, Responder};

use crate::{bot::{application::services::attendance_service, domain::table::MemberAttendanceTable}, config::database::DBPool};



pub fn show_member_attendance(
        pool: &DBPool,
        team_name: &str
    ) -> Result<Vec<MemberAttendanceTable>, String> {
    let mut conn = pool.get().expect("Failed to connect to database!");
    attendance_service::get_member_attendance(&mut conn, team_name)
}
