use crate::bot::domain::model::{NewUser, User};
use bcrypt::{hash, verify};
use chrono::Utc;
use diesel::prelude::*;

pub fn register_admin(
    conn: &mut PgConnection,
    discord_id: &str,
    username: &str,
    password: &str,
) -> Result<(), String> {
    use crate::schema::users::dsl::users;

    let hashed_password = hash(password, 4).map_err(|_| "Password hashing failed!")?;

    let new_user = NewUser {
        discord_id: discord_id.to_string(),
        username: username.to_string(),
        password_hash: hashed_password.to_string(),
        is_admin: true,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(conn)
        .map_err(|_| "Failed to insert user!")?;

    Ok(())
}

pub fn authentication_admin(
    conn: &mut PgConnection,
    dc_id: &str,
    password: &str,
) -> Result<bool, String> {
    use crate::schema::users::dsl::users;
    use crate::schema::users::*;

    let user: User = users
        .filter(discord_id.eq(dc_id))
        .first::<User>(conn)
        .map_err(|_| "User not found")?;

    Ok(verify(password, &user.password_hash).map_err(|_| "Invalid password")?)
}
