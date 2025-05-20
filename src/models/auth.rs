use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub jwt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user ID
    pub exp: usize,  // expiry
    pub email: String,
}
