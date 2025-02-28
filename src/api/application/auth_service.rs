use crate::{
    api::{domain::dto::Claims, infrastructure::auth_repository::find_by_username},
    config::database::DBPool,
};
use chrono::{Duration, Utc};
use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use std::env;

pub fn login_user(pool: &DBPool, username: &str, password: &str) -> Result<String, &'static str> {
    dotenv().ok();
    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET is not set in .env file!");

    if let Some(user) = find_by_username(pool, username) {
        log::debug!("🔍 User found: {:?}", user);

        if bcrypt::verify(password, &user.password_hash).unwrap_or(false) {
            log::info!("✅ Login successful!");

            let expiration = Utc::now()
                .checked_add_signed(Duration::hours(24))
                .expect("Invalid timestamp")
                .timestamp() as usize;

            let claims = Claims {
                sub: user.username,
                exp: expiration,
            };
            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret_key.as_ref()),
            )
            .unwrap();

            return Ok(token);
        } else {
            log::error!("❌ Password mismatch for user {}", username);
        }
    } else {
        log::error!("❌ User not found: {}", username);
    }

    Err("Invalid credentials")
}
