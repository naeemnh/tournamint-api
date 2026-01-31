use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;

use super::entity::{PublicUserProfile, User, UserProfile, UserToken};
use super::value_objects::{EditableUser, NewUser, NewUserProfile, UpdateUserProfile};
use crate::shared::AppError;

/// Repository trait for User entity operations
#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<User>, AppError>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_google_id(&self, google_id: &str) -> Result<Option<User>, AppError>;
    async fn create(&self, new_user: NewUser) -> Result<User, AppError>;
    async fn update(&self, user_id: Uuid, user_data: EditableUser) -> Result<Option<User>, AppError>;
    async fn delete(&self, user_id: Uuid) -> Result<Option<User>, AppError>;
}

/// Repository trait for UserProfile entity operations
#[async_trait]
pub trait UserProfileRepository: Send + Sync {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError>;
    async fn find_public_profile_by_user_id(&self, user_id: Uuid) -> Result<Option<PublicUserProfile>, AppError>;
    async fn create(&self, new_profile: NewUserProfile) -> Result<UserProfile, AppError>;
    async fn update(&self, user_id: Uuid, profile_data: UpdateUserProfile) -> Result<Option<UserProfile>, AppError>;
    async fn update_preferences(&self, user_id: Uuid, preferences: Value) -> Result<Option<UserProfile>, AppError>;
    async fn update_notification_preferences(&self, user_id: Uuid, notification_preferences: Value) -> Result<Option<UserProfile>, AppError>;
    async fn update_privacy_settings(&self, user_id: Uuid, privacy_settings: Value) -> Result<Option<UserProfile>, AppError>;
    async fn update_avatar(&self, user_id: Uuid, avatar_url: String) -> Result<Option<UserProfile>, AppError>;
    async fn remove_avatar(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError>;
    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError>;
}

/// Repository trait for UserToken operations
#[async_trait]
pub trait TokenRepository: Send + Sync {
    async fn upsert_refresh_token(&self, token_data: UserToken) -> Result<(), AppError>;
}
