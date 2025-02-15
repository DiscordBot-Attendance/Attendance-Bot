use crate::schema::member_attendance;
use crate::schema::members;
use crate::schema::teams;
use crate::schema::users;

use chrono::NaiveDate;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub discord_id: String,
    pub username: String,
    pub password_hash: String,
    pub is_admin: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub admin_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = teams)]
pub struct NewTeam {
    pub name: String,
    pub admin_id: i32,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = members)]
pub struct NewMember<'a> {
    pub discord_id: &'a str,
    pub team_id: i32,
    pub position: String,
    pub join_date: Option<NaiveDate>,
}
