use serde::{Deserialize, Serialize};
use tabled::Tabled;

/// Represents a team in a tabular format for display purposes.
///
/// # Fields
/// * `name` - The name of the team.
/// * `created_at` - The timestamp when the team was created, formatted as a string.
#[derive(Debug, Tabled, Serialize, Deserialize)]
pub struct TeamTable {
    pub name: String,
    pub created_at: String,
}

/// Represents a member in a tabular format for display purposes.
///
/// # Fields
/// * `username` - The username of the member.
/// * `join_date` - The date when the member joined, formatted as a string.
#[derive(Debug, Tabled, Serialize, Deserialize)]
pub struct MemberTable {
    pub username: String,
    pub join_date: String,
}

/// Represents a member's attendance record in a tabular format for display purposes.
///
/// # Fields
/// * `username` - The username of the member.
/// * `check_in_time` - The timestamp when the member checked in, formatted as a string.
/// * `check_out_time` - The timestamp when the member checked out, formatted as a string.
/// * `status` - The status of the attendance (e.g., "Present", "Late").
#[derive(Debug, Tabled, Serialize, Deserialize)]
pub struct MemberAttendanceTable {
    pub username: String,
    pub check_in_time: String,
    pub check_out_time: String,
    pub status: String,
}
