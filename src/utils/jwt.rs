use crate::models::{auth::Claims, user::User};

pub fn generate_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    use chrono::Utc;
    use jsonwebtoken::{encode, EncodingKey, Header};

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Invalid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        exp: expiration as usize,
        email: user.email.clone(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
