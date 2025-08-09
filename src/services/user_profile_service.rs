use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::config::DbPool;
use crate::formatters;
use crate::models::user_profile::{
    NewUserProfile, UpdateAvatarRequest, UpdateNotificationPreferences,
    UpdatePrivacySettings, UpdateUserPreferences, UpdateUserProfile,
};
use crate::repositories::UserProfileRepository;
use crate::utils::db::with_transaction;

pub struct UserProfileService;

impl UserProfileService {
    pub async fn get_current_user_profile(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserProfileRepository::find_by_user_id(tx, user_id).await })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "PROFILE_FOUND")
            }
            Ok(None) => {
                // If no profile exists, create a default one
                let new_profile = NewUserProfile {
                    user_id,
                    bio: None,
                    avatar_url: None,
                    phone: None,
                    date_of_birth: None,
                    timezone: None,
                    language: Some("en".to_string()),
                    notification_preferences: Some(serde_json::json!({})),
                    privacy_settings: Some(serde_json::json!({})),
                    location: None,
                    website: None,
                    social_links: Some(serde_json::json!({})),
                    preferences: Some(serde_json::json!({})),
                    is_public: Some(true),
                };

                match with_transaction(pool, |tx| {
                    Box::pin(async move { UserProfileRepository::create(tx, new_profile).await })
                })
                .await
                {
                    Ok(profile) => {
                        formatters::success_response(StatusCode::CREATED, profile, "PROFILE_CREATED")
                    }
                    Err(e) => formatters::error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &e.to_string(),
                        "PROFILE_CREATION_ERROR",
                    ),
                }
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn get_public_user_profile(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                UserProfileRepository::find_public_profile_by_user_id(tx, user_id).await
            })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "PUBLIC_PROFILE_FOUND")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Profile not found or not public",
                "PROFILE_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn update_user_profile(
        pool: &DbPool,
        user_id: Uuid,
        profile_data: UpdateUserProfile,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                UserProfileRepository::update(tx, user_id, profile_data).await
            })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "PROFILE_UPDATED")
            }
            Ok(None) => {
                // If no profile exists, create one first with default values, then update
                let new_profile = NewUserProfile {
                    user_id,
                    bio: None,
                    avatar_url: None,
                    phone: None,
                    date_of_birth: None,
                    timezone: None,
                    language: Some("en".to_string()),
                    notification_preferences: Some(serde_json::json!({})),
                    privacy_settings: Some(serde_json::json!({})),
                    location: None,
                    website: None,
                    social_links: Some(serde_json::json!({})),
                    preferences: Some(serde_json::json!({})),
                    is_public: Some(true),
                };

                match with_transaction(pool, |tx| {
                    Box::pin(async move { UserProfileRepository::create(tx, new_profile).await })
                })
                .await
                {
                    Ok(profile) => {
                        formatters::success_response(StatusCode::CREATED, profile, "PROFILE_CREATED")
                    }
                    Err(e) => formatters::error_response(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        &e.to_string(),
                        "PROFILE_CREATION_ERROR",
                    ),
                }
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn update_user_preferences(
        pool: &DbPool,
        user_id: Uuid,
        preferences_data: UpdateUserPreferences,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                UserProfileRepository::update_preferences(tx, user_id, preferences_data.preferences)
                    .await
            })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "PREFERENCES_UPDATED")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Profile not found",
                "PROFILE_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn update_notification_preferences(
        pool: &DbPool,
        user_id: Uuid,
        notification_data: UpdateNotificationPreferences,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                UserProfileRepository::update_notification_preferences(
                    tx,
                    user_id,
                    notification_data.notification_preferences,
                )
                .await
            })
        })
        .await
        {
            Ok(Some(profile)) => formatters::success_response(
                StatusCode::OK,
                profile,
                "NOTIFICATION_PREFERENCES_UPDATED",
            ),
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Profile not found",
                "PROFILE_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn update_privacy_settings(
        pool: &DbPool,
        user_id: Uuid,
        privacy_data: UpdatePrivacySettings,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                UserProfileRepository::update_privacy_settings(
                    tx,
                    user_id,
                    privacy_data.privacy_settings,
                )
                .await
            })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "PRIVACY_SETTINGS_UPDATED")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Profile not found",
                "PROFILE_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn update_avatar(
        pool: &DbPool,
        user_id: Uuid,
        avatar_data: UpdateAvatarRequest,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                UserProfileRepository::update_avatar(tx, user_id, avatar_data.avatar_url).await
            })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "AVATAR_UPDATED")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Profile not found",
                "PROFILE_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }

    pub async fn remove_avatar(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserProfileRepository::remove_avatar(tx, user_id).await })
        })
        .await
        {
            Ok(Some(profile)) => {
                formatters::success_response(StatusCode::OK, profile, "AVATAR_REMOVED")
            }
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Profile not found",
                "PROFILE_NOT_FOUND",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNKNOWN_ERROR",
            ),
        }
    }
}