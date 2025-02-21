use dotenvy::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// Represents the claims (payload) of a JWT token.
///
/// # Fields
/// * `sub` - The subject of the token (e.g., the Discord ID of the user).
/// * `exp` - The expiration time of the token (in seconds since the Unix epoch).
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

/// Generates a JWT token for a given Discord ID.
///
/// # Arguments
/// * `discord_id` - The Discord ID of the user for whom the token is being generated.
///
/// # Returns
/// Returns a JWT token as a `String`.
///
/// # Panics
/// Panics if:
/// - The `.env` file cannot be loaded.
/// - The `JWT_SECRET` environment variable is not set.
/// - Token creation fails.
pub fn generate_token(discord_id: &str) -> String {
    // Load environment variables from the `.env` file.
    dotenv().ok();

    // Retrieve the JWT secret from the environment variables.
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Calculate the expiration time (1 hour from now).
    let expiration = SystemTime::now()
        .checked_add(Duration::from_secs(3600))
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // Define the claims (payload) for the token.
    let claims = Claims {
        sub: discord_id.to_string(),
        exp: expiration,
    };

    // Encode the token using the JWT secret.
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .expect("Token failed to be created!")
}
