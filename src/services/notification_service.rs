use actix_web::{HttpResponse, web};
use uuid::Uuid;

use crate::config::DbPool;
use crate::models::notification::{
    NewNotification, Notification, NotificationCount, SendNotificationRequest
};
use crate::repositories::notification_repository::NotificationRepository;

pub struct NotificationService;

impl NotificationService {
    pub async fn get_notifications(
        pool: &DbPool,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let limit = limit.unwrap_or(50).min(100); // Cap at 100
        let offset = offset.unwrap_or(0);

        match NotificationRepository::get_by_user_id(pool, user_id, limit, offset).await {
            Ok(notifications) => HttpResponse::Ok().json(notifications),
            Err(err) => {
                eprintln!("Error fetching notifications: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch notifications"
                }))
            }
        }
    }

    pub async fn get_unread_notifications(
        pool: &DbPool,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> HttpResponse {
        let limit = limit.unwrap_or(50).min(100); // Cap at 100
        let offset = offset.unwrap_or(0);

        match NotificationRepository::get_unread_by_user_id(pool, user_id, limit, offset).await {
            Ok(notifications) => HttpResponse::Ok().json(notifications),
            Err(err) => {
                eprintln!("Error fetching unread notifications: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch unread notifications"
                }))
            }
        }
    }

    pub async fn mark_as_read(
        pool: &DbPool,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> HttpResponse {
        match NotificationRepository::mark_as_read(pool, notification_id, user_id).await {
            Ok(notification) => HttpResponse::Ok().json(notification),
            Err(sqlx::Error::RowNotFound) => {
                HttpResponse::NotFound().json(serde_json::json!({
                    "error": "Notification not found"
                }))
            }
            Err(err) => {
                eprintln!("Error marking notification as read: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to mark notification as read"
                }))
            }
        }
    }

    pub async fn mark_all_as_read(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match NotificationRepository::mark_all_as_read(pool, user_id).await {
            Ok(affected_rows) => HttpResponse::Ok().json(serde_json::json!({
                "message": "All notifications marked as read",
                "affected_count": affected_rows
            })),
            Err(err) => {
                eprintln!("Error marking all notifications as read: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to mark all notifications as read"
                }))
            }
        }
    }

    pub async fn delete_notification(
        pool: &DbPool,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> HttpResponse {
        match NotificationRepository::delete(pool, notification_id, user_id).await {
            Ok(affected_rows) => {
                if affected_rows > 0 {
                    HttpResponse::Ok().json(serde_json::json!({
                        "message": "Notification deleted successfully"
                    }))
                } else {
                    HttpResponse::NotFound().json(serde_json::json!({
                        "error": "Notification not found"
                    }))
                }
            }
            Err(err) => {
                eprintln!("Error deleting notification: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to delete notification"
                }))
            }
        }
    }

    pub async fn get_unread_count(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match NotificationRepository::get_unread_count(pool, user_id).await {
            Ok(count) => {
                let notification_count = NotificationCount {
                    unread_count: count,
                };
                HttpResponse::Ok().json(notification_count)
            }
            Err(err) => {
                eprintln!("Error fetching unread count: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to fetch unread count"
                }))
            }
        }
    }

    pub async fn send_notification(
        pool: &DbPool,
        request: SendNotificationRequest,
    ) -> HttpResponse {
        if request.user_ids.is_empty() {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "User IDs list cannot be empty"
            }));
        }

        let notifications: Vec<NewNotification> = request
            .user_ids
            .iter()
            .map(|&user_id| NewNotification {
                user_id,
                title: request.title.clone(),
                message: request.message.clone(),
                notification_type: request.notification_type.clone(),
                tournament_id: request.tournament_id,
                match_id: request.match_id,
            })
            .collect();

        match NotificationRepository::create_bulk(pool, &notifications).await {
            Ok(created_notifications) => HttpResponse::Created().json(serde_json::json!({
                "message": "Notifications sent successfully",
                "count": created_notifications.len(),
                "notifications": created_notifications
            })),
            Err(err) => {
                eprintln!("Error sending notifications: {:?}", err);
                HttpResponse::InternalServerError().json(serde_json::json!({
                    "error": "Failed to send notifications"
                }))
            }
        }
    }

    pub async fn create_notification(
        pool: &DbPool,
        notification: NewNotification,
    ) -> Result<Notification, String> {
        NotificationRepository::create(pool, &notification)
            .await
            .map_err(|err| {
                eprintln!("Error creating notification: {:?}", err);
                "Failed to create notification".to_string()
            })
    }
}
