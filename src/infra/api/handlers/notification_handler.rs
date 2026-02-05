use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::application::NotificationServices;
use crate::domain::notification::NewNotification;
use crate::infra::api::middleware::auth::get_user_id_from_request;
use crate::infra::db::PgNotificationRepository;
use crate::shared::ApiResponse;

type NotificationServicesData = std::sync::Arc<NotificationServices<PgNotificationRepository>>;

pub struct NotificationHandler;

impl NotificationHandler {
    pub async fn index(
        services: web::Data<NotificationServicesData>,
        req: HttpRequest,
        query: web::Query<NotificationQuery>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0).max(0);
        match services.get_notifications(user_id, limit, offset).await {
            Ok(notifications) => ApiResponse::success("OK", Some(notifications)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_unread(
        services: web::Data<NotificationServicesData>,
        req: HttpRequest,
        query: web::Query<NotificationQuery>,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        let limit = query.limit.unwrap_or(20).min(100);
        let offset = query.offset.unwrap_or(0).max(0);
        match services
            .get_unread_notifications(user_id, limit, offset)
            .await
        {
            Ok(notifications) => ApiResponse::success("OK", Some(notifications)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_count(
        services: web::Data<NotificationServicesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.get_unread_count(user_id).await {
            Ok(count) => ApiResponse::success("OK", Some(serde_json::json!({ "count": count }))),
            Err(e) => e.error_response(),
        }
    }

    pub async fn mark_all_read(
        services: web::Data<NotificationServicesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.mark_all_as_read(user_id).await {
            Ok(count) => {
                ApiResponse::success("Updated", Some(serde_json::json!({ "updated": count })))
            }
            Err(e) => e.error_response(),
        }
    }

    pub async fn send(
        services: web::Data<NotificationServicesData>,
        body: web::Json<NewNotification>,
    ) -> HttpResponse {
        match services.send_notification(body.into_inner()).await {
            Ok(notification) => ApiResponse::created("Sent", notification),
            Err(e) => e.error_response(),
        }
    }

    pub async fn mark_read(
        services: web::Data<NotificationServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.mark_as_read(id).await {
            Ok(Some(n)) => ApiResponse::success("Updated", Some(n)),
            Ok(None) => ApiResponse::not_found("Notification not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        services: web::Data<NotificationServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.delete_notification(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Notification not found"),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct NotificationQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
