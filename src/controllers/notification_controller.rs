use actix_web::{web, Responder, HttpRequest};
use uuid::Uuid;

use crate::config::DbPool;
use crate::models::notification::SendNotificationRequest;
use crate::services::notification_service::NotificationService;
use crate::middlewares::auth::get_user_from_token;

pub struct NotificationController;

impl NotificationController {
    /// GET /notifications - Get user's notifications
    pub async fn index(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => {
                NotificationService::get_notifications(
                    &pool,
                    user.id,
                    query.limit,
                    query.offset,
                )
                .await
            }
            Err(response) => response,
        }
    }

    /// GET /notifications/unread - Get unread notifications
    pub async fn get_unread(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        query: web::Query<PaginationQuery>,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => {
                NotificationService::get_unread_notifications(
                    &pool,
                    user.id,
                    query.limit,
                    query.offset,
                )
                .await
            }
            Err(response) => response,
        }
    }

    /// PUT /notifications/{id}/read - Mark as read
    pub async fn mark_as_read(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let notification_id = path.into_inner();
        
        match get_user_from_token(&req).await {
            Ok(user) => {
                NotificationService::mark_as_read(&pool, notification_id, user.id).await
            }
            Err(response) => response,
        }
    }

    /// PUT /notifications/read-all - Mark all as read
    pub async fn mark_all_as_read(
        pool: web::Data<DbPool>,
        req: HttpRequest,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => NotificationService::mark_all_as_read(&pool, user.id).await,
            Err(response) => response,
        }
    }

    /// DELETE /notifications/{id} - Delete notification
    pub async fn delete(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        path: web::Path<Uuid>,
    ) -> impl Responder {
        let notification_id = path.into_inner();
        
        match get_user_from_token(&req).await {
            Ok(user) => {
                NotificationService::delete_notification(&pool, notification_id, user.id).await
            }
            Err(response) => response,
        }
    }

    /// GET /notifications/count - Get unread count
    pub async fn get_unread_count(
        pool: web::Data<DbPool>,
        req: HttpRequest,
    ) -> impl Responder {
        match get_user_from_token(&req).await {
            Ok(user) => NotificationService::get_unread_count(&pool, user.id).await,
            Err(response) => response,
        }
    }

    /// POST /notifications/send - Send notification (admin only)
    pub async fn send_notification(
        pool: web::Data<DbPool>,
        req: HttpRequest,
        request: web::Json<SendNotificationRequest>,
    ) -> impl Responder {
        // TODO: Add admin role check when user roles are implemented
        match get_user_from_token(&req).await {
            Ok(_user) => {
                // For now, allow any authenticated user to send notifications
                // In production, this should check for admin role
                NotificationService::send_notification(&pool, request.into_inner()).await
            }
            Err(response) => response,
        }
    }
}

#[derive(serde::Deserialize)]
struct PaginationQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}
