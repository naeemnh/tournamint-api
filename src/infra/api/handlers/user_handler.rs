use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::domain::user::{EditableUser, NewUser, UpdateUserProfile};
use crate::shared::ApiResponse;
use crate::infra::api::middleware::auth::get_user_id_from_request;

/// User handlers - placeholder showing the pattern
/// In full implementation, these would use injected use cases via app_data
pub struct UserHandler;

impl UserHandler {
    pub async fn index() -> HttpResponse {
        // TODO: Get use cases from app_data and call get_all_users
        ApiResponse::error("Not implemented - inject use cases via app_data")
    }

    pub async fn show(path: web::Path<Uuid>) -> HttpResponse {
        let _user_id = path.into_inner();
        ApiResponse::error("Not implemented - inject use cases via app_data")
    }

    pub async fn post(_body: web::Json<NewUser>) -> HttpResponse {
        ApiResponse::error("Not implemented - inject use cases via app_data")
    }

    pub async fn update(
        path: web::Path<Uuid>,
        _body: web::Json<EditableUser>,
    ) -> HttpResponse {
        let _user_id = path.into_inner();
        ApiResponse::error("Not implemented - inject use cases via app_data")
    }

    pub async fn delete(path: web::Path<Uuid>) -> HttpResponse {
        let _user_id = path.into_inner();
        ApiResponse::error("Not implemented - inject use cases via app_data")
    }
}

/// User profile handlers
pub struct UserProfileHandler;

impl UserProfileHandler {
    pub async fn get_current_profile(req: HttpRequest) -> HttpResponse {
        match get_user_id_from_request(&req) {
            Ok(_user_id) => {
                // TODO: Get profile use cases from app_data
                ApiResponse::error("Not implemented - inject use cases via app_data")
            }
            Err(response) => response,
        }
    }

    pub async fn update_profile(
        req: HttpRequest,
        _body: web::Json<UpdateUserProfile>,
    ) -> HttpResponse {
        match get_user_id_from_request(&req) {
            Ok(_user_id) => {
                ApiResponse::error("Not implemented - inject use cases via app_data")
            }
            Err(response) => response,
        }
    }

    pub async fn get_public_profile(path: web::Path<Uuid>) -> HttpResponse {
        let _user_id = path.into_inner();
        ApiResponse::error("Not implemented - inject use cases via app_data")
    }
}
