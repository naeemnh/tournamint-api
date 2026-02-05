use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::application::UserServices;
use crate::domain::user::{
    EditableUser, NewUser, UpdateNotificationPreferences, UpdatePrivacySettings,
    UpdateUserPreferences, UpdateUserProfile,
};
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::api::multipart_util::extract_file_from_multipart;
use crate::infra::cloudinary::CloudinaryClient;
use crate::infra::db::{PgUserProfileRepository, PgUserRepository};
use crate::shared::ApiResponse;

/// Concrete type for dependency injection
type UserServicesData = std::sync::Arc<UserServices<PgUserRepository, PgUserProfileRepository>>;

pub struct UserHandler;

impl UserHandler {
    pub async fn index(services: web::Data<UserServicesData>) -> HttpResponse {
        match services.find_all().await {
            Ok(users) => ApiResponse::success("OK", Some(users)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        services: web::Data<UserServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match services.find_by_id(user_id).await {
            Ok(Some(user)) => ApiResponse::success("OK", Some(user)),
            Ok(None) => ApiResponse::not_found("User not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn post(
        services: web::Data<UserServicesData>,
        body: web::Json<NewUser>,
    ) -> HttpResponse {
        match services.create(body.into_inner()).await {
            Ok(user) => ApiResponse::created("Created", user),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        services: web::Data<UserServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableUser>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match services.update(user_id, body.into_inner()).await {
            Ok(Some(user)) => ApiResponse::success("Updated", Some(user)),
            Ok(None) => ApiResponse::not_found("User not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        services: web::Data<UserServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match services.delete(user_id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("User not found"),
            Err(e) => e.error_response(),
        }
    }
}

pub struct UserProfileHandler;

impl UserProfileHandler {
    pub async fn get_current_profile(
        services: web::Data<UserServicesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.find_profile_by_user_id(user_id).await {
            Ok(Some(profile)) => ApiResponse::success("OK", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_profile(
        services: web::Data<UserServicesData>,
        req: HttpRequest,
        body: web::Json<UpdateUserProfile>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.update_profile(user_id, body.into_inner()).await {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_public_profile(
        services: web::Data<UserServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match services.find_public_profile_by_user_id(user_id).await {
            Ok(Some(profile)) => ApiResponse::success("OK", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_preferences(
        services: web::Data<UserServicesData>,
        req: HttpRequest,
        body: web::Json<UpdateUserPreferences>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services
            .update_preferences(user_id, body.preferences.clone())
            .await
        {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_notification_preferences(
        services: web::Data<UserServicesData>,
        req: HttpRequest,
        body: web::Json<UpdateNotificationPreferences>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services
            .update_notification_preferences(user_id, body.notification_preferences.clone())
            .await
        {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_privacy_settings(
        services: web::Data<UserServicesData>,
        req: HttpRequest,
        body: web::Json<UpdatePrivacySettings>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services
            .update_privacy_settings(user_id, body.privacy_settings.clone())
            .await
        {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_avatar(
        services: web::Data<UserServicesData>,
        cloudinary: web::Data<std::sync::Arc<CloudinaryClient>>,
        req: HttpRequest,
        mut payload: Multipart,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let bytes = match extract_file_from_multipart(&mut payload, 10 * 1024 * 1024).await {
            Ok(b) => b,
            Err(r) => return r,
        };
        let public_id = format!("tournamint/avatars/{}", user_id);
        let result = match cloudinary.upload(&bytes, "image", &public_id).await {
            Ok(r) => r,
            Err(e) => return e.error_response(),
        };
        match services.update_avatar(user_id, result.secure_url).await {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn remove_avatar(
        services: web::Data<UserServicesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.remove_avatar(user_id).await {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }
}
