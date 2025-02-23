use crate::bot::domain::model::{NewUser, User};
use bcrypt::{hash, verify};
use chrono::Utc;
use diesel::prelude::*;

/// Registers a new admin user in the database.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `discord_id` - The Discord ID of the user.
/// * `username` - The username of the user.
/// * `password` - The password of the user.
///
/// # Errors
/// Returns an error if:
/// - Password hashing fails.
/// - Inserting the user into the database fails.
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

/// Authenticates an admin user by verifying their password.
///
/// # Arguments
/// * `conn` - A mutable reference to the PostgreSQL connection.
/// * `dc_id` - The Discord ID of the user.
/// * `password` - The password to verify.
///
/// # Returns
/// Returns `Ok(true)` if the password is valid, otherwise `Ok(false)`.
///
/// # Errors
/// Returns an error if:
/// - The user is not found in the database.
/// - Password verification fails.
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
