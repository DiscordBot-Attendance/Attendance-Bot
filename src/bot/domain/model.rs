use crate::schema::member_attendance;
use crate::schema::members;
use crate::schema::teams;
use crate::schema::users;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a user in the database.
///
/// # Fields
/// * `id` - The unique identifier for the user.
/// * `discord_id` - The Discord ID of the user.
/// * `username` - The username of the user.
/// * `password_hash` - The hashed password of the user.
/// * `is_admin` - Indicates whether the user has admin privileges.
/// * `created_at` - The timestamp when the user was created.
/// * `updated_at` - The timestamp when the user was last updated.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub discord_id: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: Option<bool>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// Represents a new user to be inserted into the database.
///
/// # Fields
/// * `discord_id` - The Discord ID of the user.
/// * `username` - The username of the user.
/// * `password_hash` - The hashed password of the user.
/// * `is_admin` - Indicates whether the user has admin privileges.
/// * `created_at` - The timestamp when the user was created.
#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub discord_id: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents a team in the database.
///
/// # Fields
/// * `id` - The unique identifier for the team.
/// * `name` - The name of the team.
/// * `admin_id` - The ID of the admin who created the team.
/// * `created_at` - The timestamp when the team was created.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub admin_id: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
}

/// Represents a new team to be inserted into the database.
///
/// # Fields
/// * `name` - The name of the team.
/// * `admin_id` - The ID of the admin who created the team.
/// * `created_at` - The timestamp when the team was created.
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub name: String,
    pub admin_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

/// Represents a member in the database.
///
/// # Fields
/// * `id` - The unique identifier for the member.
/// * `team_id` - The ID of the team the member belongs to.
/// * `username` - The username of the member.
/// * `discord_id` - The Discord ID of the member.
/// * `position` - The position or role of the member in the team.
/// * `join_date` - The date when the member joined the team.
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Member {
    pub id: i32,
    pub team_id: Option<i32>,
    pub username: String,
    pub discord_id: String,
    pub position: Option<String>,
    pub join_date: Option<NaiveDate>,
}

/// Represents a new member to be inserted into the database.
///
/// # Fields
/// * `discord_id` - The Discord ID of the member.
/// * `username` - The username of the member.
/// * `team_id` - The ID of the team the member belongs to.
/// * `position` - The position or role of the member in the team.
/// * `join_date` - The date when the member joined the team.
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = members)]
pub struct NewMember<'a> {
    pub discord_id: &'a str,
    pub username: String,
    pub team_id: i32,
    pub position: String,
    pub join_date: Option<NaiveDate>,
}

/// Represents a member's attendance record in the database.
///
/// # Fields
/// * `id` - The unique identifier for the attendance record.
/// * `team_id` - The ID of the team the member belongs to.
/// * `member_id` - The ID of the member.
/// * `date` - The date of the attendance record.
/// * `check_in_time` - The timestamp when the member checked in.
/// * `check_out_time` - The timestamp when the member checked out.
/// * `status` - The status of the attendance (e.g., "Present", "Late").
#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct MemberAttendance {
    pub id: i32,
    pub team_id: Option<i32>,
    pub member_id: Option<i32>,
    pub date: NaiveDate,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub status: Option<String>,
}

/// Represents a new check-in attendance record to be inserted into the database.
///
/// # Fields
/// * `member_id` - The ID of the member checking in.
/// * `team_id` - The ID of the team the member belongs to.
/// * `date` - The date of the attendance record.
/// * `check_in_time` - The timestamp when the member checked in.
/// * `status` - The status of the attendance (e.g., "Present", "Late").
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = member_attendance)]
pub struct CheckInAttendance {
    pub member_id: i32,
    pub team_id: i32,
    pub date: NaiveDate,
    pub check_in_time: NaiveDateTime,
    pub status: String,
}
