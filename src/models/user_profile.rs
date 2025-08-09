use std::fmt::Write;
use chrono::{DateTime, NaiveDate, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserPreferences {
    pub preferences: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateNotificationPreferences {
    pub notification_preferences: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePrivacySettings {
    pub privacy_settings: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAvatarRequest {
    pub avatar_url: String,
}

pub enum UserProfileIden {
    Table,
    Id,
    UserId,
    Bio,
    AvatarUrl,
    Phone,
    DateOfBirth,
    Timezone,
    Language,
    NotificationPreferences,
    PrivacySettings,
    Location,
    Website,
    SocialLinks,
    Preferences,
    IsPublic,
    CreatedAt,
    UpdatedAt,
}

impl Iden for UserProfileIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                UserProfileIden::Table => "user_profiles",
                UserProfileIden::Id => "id",
                UserProfileIden::UserId => "user_id",
                UserProfileIden::Bio => "bio",
                UserProfileIden::AvatarUrl => "avatar_url",
                UserProfileIden::Phone => "phone",
                UserProfileIden::DateOfBirth => "date_of_birth",
                UserProfileIden::Timezone => "timezone",
                UserProfileIden::Language => "language",
                UserProfileIden::NotificationPreferences => "notification_preferences",
                UserProfileIden::PrivacySettings => "privacy_settings",
                UserProfileIden::Location => "location",
                UserProfileIden::Website => "website",
                UserProfileIden::SocialLinks => "social_links",
                UserProfileIden::Preferences => "preferences",
                UserProfileIden::IsPublic => "is_public",
                UserProfileIden::CreatedAt => "created_at",
                UserProfileIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}