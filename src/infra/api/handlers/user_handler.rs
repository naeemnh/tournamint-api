use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::application::UserUseCases;
use crate::domain::user::{
    EditableUser, NewUser, UpdateAvatarRequest, UpdateNotificationPreferences,
    UpdatePrivacySettings, UpdateUserPreferences, UpdateUserProfile,
};
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::db::{PgUserProfileRepository, PgUserRepository};
use crate::shared::ApiResponse;

/// Concrete type for dependency injection
type UserUseCasesData = std::sync::Arc<UserUseCases<PgUserRepository, PgUserProfileRepository>>;

pub struct UserHandler;

impl UserHandler {
    pub async fn index(use_cases: web::Data<UserUseCasesData>) -> HttpResponse {
        match use_cases.find_all().await {
            Ok(users) => ApiResponse::success("OK", Some(users)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        use_cases: web::Data<UserUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match use_cases.find_by_id(user_id).await {
            Ok(Some(user)) => ApiResponse::success("OK", Some(user)),
            Ok(None) => ApiResponse::not_found("User not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn post(
        use_cases: web::Data<UserUseCasesData>,
        body: web::Json<NewUser>,
    ) -> HttpResponse {
        match use_cases.create(body.into_inner()).await {
            Ok(user) => ApiResponse::created("Created", user),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<UserUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableUser>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match use_cases.update(user_id, body.into_inner()).await {
            Ok(Some(user)) => ApiResponse::success("Updated", Some(user)),
            Ok(None) => ApiResponse::not_found("User not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<UserUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match use_cases.delete(user_id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("User not found"),
            Err(e) => e.error_response(),
        }
    }
}

pub struct UserProfileHandler;

impl UserProfileHandler {
    pub async fn get_current_profile(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.find_profile_by_user_id(user_id).await {
            Ok(Some(profile)) => ApiResponse::success("OK", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_profile(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
        body: web::Json<UpdateUserProfile>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.update_profile(user_id, body.into_inner()).await {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_public_profile(
        use_cases: web::Data<UserUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let user_id = path.into_inner();
        match use_cases.find_public_profile_by_user_id(user_id).await {
            Ok(Some(profile)) => ApiResponse::success("OK", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_preferences(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
        body: web::Json<UpdateUserPreferences>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases
            .update_preferences(user_id, body.preferences.clone())
            .await
        {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_notification_preferences(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
        body: web::Json<UpdateNotificationPreferences>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases
            .update_notification_preferences(user_id, body.notification_preferences.clone())
            .await
        {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_privacy_settings(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
        body: web::Json<UpdatePrivacySettings>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases
            .update_privacy_settings(user_id, body.privacy_settings.clone())
            .await
        {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_avatar(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
        body: web::Json<UpdateAvatarRequest>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.update_avatar(user_id, body.avatar_url.clone()).await {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn remove_avatar(
        use_cases: web::Data<UserUseCasesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.remove_avatar(user_id).await {
            Ok(Some(profile)) => ApiResponse::success("Updated", Some(profile)),
            Ok(None) => ApiResponse::not_found("Profile not found"),
            Err(e) => e.error_response(),
        }
    }
}
