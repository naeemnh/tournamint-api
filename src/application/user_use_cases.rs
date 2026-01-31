use std::sync::Arc;
use serde_json::Value;
use uuid::Uuid;

use crate::domain::user::{
    EditableUser, NewUser, NewUserProfile, PublicUserProfile, UpdateUserProfile, User,
    UserProfile, UserProfileRepository, UserRepository,
};
use crate::shared::AppError;

/// User and profile use cases (CRUD and profile operations)
pub struct UserUseCases<U, P>
where
    U: UserRepository,
    P: UserProfileRepository,
{
    user_repo: Arc<U>,
    profile_repo: Arc<P>,
}

impl<U, P> UserUseCases<U, P>
where
    U: UserRepository,
    P: UserProfileRepository,
{
    pub fn new(user_repo: Arc<U>, profile_repo: Arc<P>) -> Self {
        Self {
            user_repo,
            profile_repo,
        }
    }

    // ==================== User CRUD ====================

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        self.user_repo.find_all().await
    }

    pub async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        self.user_repo.find_by_id(user_id).await
    }

    pub async fn create(&self, new_user: NewUser) -> Result<User, AppError> {
        self.user_repo.create(new_user).await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        user_data: EditableUser,
    ) -> Result<Option<User>, AppError> {
        self.user_repo.update(user_id, user_data).await
    }

    pub async fn delete(&self, user_id: Uuid) -> Result<Option<User>, AppError> {
        self.user_repo.delete(user_id).await
    }

    // ==================== Profile ====================

    pub async fn find_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo.find_by_user_id(user_id).await
    }

    pub async fn find_public_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<PublicUserProfile>, AppError> {
        self.profile_repo.find_public_profile_by_user_id(user_id).await
    }

    pub async fn create_profile(&self, new_profile: NewUserProfile) -> Result<UserProfile, AppError> {
        self.profile_repo.create(new_profile).await
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        profile_data: UpdateUserProfile,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo.update(user_id, profile_data).await
    }

    pub async fn update_preferences(
        &self,
        user_id: Uuid,
        preferences: Value,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo.update_preferences(user_id, preferences).await
    }

    pub async fn update_notification_preferences(
        &self,
        user_id: Uuid,
        notification_preferences: Value,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo
            .update_notification_preferences(user_id, notification_preferences)
            .await
    }

    pub async fn update_privacy_settings(
        &self,
        user_id: Uuid,
        privacy_settings: Value,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo
            .update_privacy_settings(user_id, privacy_settings)
            .await
    }

    pub async fn update_avatar(
        &self,
        user_id: Uuid,
        avatar_url: String,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo.update_avatar(user_id, avatar_url).await
    }

    pub async fn remove_avatar(&self, user_id: Uuid) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo.remove_avatar(user_id).await
    }

    pub async fn delete_profile_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserProfile>, AppError> {
        self.profile_repo.delete_by_user_id(user_id).await
    }
}
