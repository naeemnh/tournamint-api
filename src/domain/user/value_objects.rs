use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::entity::User;

/// Data for creating a new user
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub name: Option<String>,
    pub email: String,
    pub google_id: String,
}

/// Data for updating an existing user
#[derive(Debug, Serialize, Deserialize)]
pub struct EditableUser {
    pub name: String,
    pub email: String,
}

/// User info received from Google OAuth
#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub email: String,
    pub name: Option<String>,
}

/// Response after successful login
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub jwt: String,
}

/// Data for creating a new user profile
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUserProfile {
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
}

/// Data for updating a user profile
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserProfile {
    pub bio: Option<String>,
    pub phone: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub location: Option<String>,
    pub website: Option<String>,
    pub social_links: Option<Value>,
    pub is_public: Option<bool>,
}

/// Request to update user preferences
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPreferences {
    pub preferences: Value,
}

/// Request to update notification preferences
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationPreferences {
    pub notification_preferences: Value,
}

/// Request to update privacy settings
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePrivacySettings {
    pub privacy_settings: Value,
}

/// Request to update avatar
#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAvatarRequest {
    pub avatar_url: String,
}
