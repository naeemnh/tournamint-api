use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Core user entity representing an authenticated user in the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub google_id: String,
    pub email: String,
    pub name: Option<String>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

/// Extended user profile with preferences and settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub notification_preferences: Option<Value>,
    pub privacy_settings: Option<Value>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<Value>,
    pub preferences: Option<Value>,
    pub is_public: Option<bool>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

/// Public-facing user profile (limited fields for privacy)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicUserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<Value>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

/// Refresh token for JWT authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserToken {
    pub refresh_token: String,
    pub user_id: Uuid,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub expires_at: DateTime<Utc>,
}
