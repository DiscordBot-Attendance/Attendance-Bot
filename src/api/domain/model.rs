use diesel::{
    prelude::{Insertable, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub is_admin: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}
