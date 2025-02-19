use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct TeamTable {
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Tabled)]
pub struct MemberTable {
    pub username: String,
    pub join_date: String,
}

#[derive(Debug, Tabled)]
pub struct MemberAttendanceTable {
    pub username: String,
    pub check_in_time: String,
    pub check_out_time: String,
    pub status: String,
}
