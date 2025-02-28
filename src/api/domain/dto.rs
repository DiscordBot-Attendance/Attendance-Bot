use serde::{Deserialize, Serialize};

// DTO for user login claim
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}


// login request
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

// login response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

