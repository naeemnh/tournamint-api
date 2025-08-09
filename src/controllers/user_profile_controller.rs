use actix_web::{web, HttpMessage, HttpRequest, Responder};
use uuid::Uuid;

use crate::config::DbPool;
use crate::models::auth::Claims;
use crate::models::user_profile::{
    UpdateAvatarRequest, UpdateNotificationPreferences, UpdatePrivacySettings,
    UpdateUserPreferences, UpdateUserProfile,
};
use crate::services::UserProfileService;

pub struct UserProfileController;

impl UserProfileController {
    /// GET /profile - Get current user's profile
    pub async fn get_current_profile(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
        let req_extentions = req.extensions();
        // Extract user ID from JWT claims (set by auth middleware)
        let claims = req_extentions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::get_current_user_profile(&pool, user_id).await
    }

    /// PUT /profile - Update current user's profile
    pub async fn update_current_profile(
        pool: web::Data<DbPool>,
        profile_data: web::Json<UpdateUserProfile>,
        req: HttpRequest,
    ) -> impl Responder {
        let req_extensions = req.extensions();
        let claims = req_extensions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::update_user_profile(&pool, user_id, profile_data.into_inner()).await
    }

    /// POST /profile/preferences - Update user preferences
    pub async fn update_preferences(
        pool: web::Data<DbPool>,
        preferences_data: web::Json<UpdateUserPreferences>,
        req: HttpRequest,
    ) -> impl Responder {
        let req_extensions = req.extensions();
        let claims = req_extensions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::update_user_preferences(&pool, user_id, preferences_data.into_inner())
            .await
    }

    /// POST /profile/notifications - Update notification preferences
    pub async fn update_notification_preferences(
        pool: web::Data<DbPool>,
        notification_data: web::Json<UpdateNotificationPreferences>,
        req: HttpRequest,
    ) -> impl Responder {
        let req_extensions = req.extensions();
        let claims = req_extensions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::update_notification_preferences(
            &pool,
            user_id,
            notification_data.into_inner(),
        )
        .await
    }

    /// POST /profile/privacy - Update privacy settings
    pub async fn update_privacy_settings(
        pool: web::Data<DbPool>,
        privacy_data: web::Json<UpdatePrivacySettings>,
        req: HttpRequest,
    ) -> impl Responder {
        let req_extensions = req.extensions();
        let claims = req_extensions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::update_privacy_settings(&pool, user_id, privacy_data.into_inner()).await
    }

    /// POST /profile/avatar - Upload/update avatar
    pub async fn update_avatar(
        pool: web::Data<DbPool>,
        avatar_data: web::Json<UpdateAvatarRequest>,
        req: HttpRequest,
    ) -> impl Responder {
        let req_extensions = req.extensions();
        let claims = req_extensions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::update_avatar(&pool, user_id, avatar_data.into_inner()).await
    }

    /// DELETE /profile/avatar - Remove avatar
    pub async fn remove_avatar(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
        let req_extensions = req.extensions();
        let claims = req_extensions.get::<Claims>().unwrap().clone();
        let user_id = Uuid::parse_str(&claims.sub).unwrap();

        UserProfileService::remove_avatar(&pool, user_id).await
    }

    /// GET /profile/{user_id} - Get another user's public profile
    pub async fn get_public_profile(
        pool: web::Data<DbPool>,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let user_id = path.into_inner();
        UserProfileService::get_public_user_profile(&pool, user_id).await
    }
}
