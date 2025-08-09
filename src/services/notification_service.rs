use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::notification::{NewNotification, NotificationCount, SendNotificationRequest},
    repositories::notification_repository::NotificationRepository,
    utils::db::with_transaction,
};

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

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                NotificationRepository::get_by_user_id(tx, user_id, limit, offset).await
            })
        })
        .await
        {
            Ok(notifications) => {
                formatters::success_response(StatusCode::OK, notifications, "NOTIFICATIONS_FETCHED")
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "NOTIFICATIONS_FETCH_ERROR",
            ),
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

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                NotificationRepository::get_unread_by_user_id(tx, user_id, limit, offset).await
            })
        })
        .await
        {
            Ok(notifications) => formatters::success_response(
                StatusCode::OK,
                notifications,
                "UNREAD_NOTIFICATIONS_FETCHED",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNREAD_NOTIFICATIONS_FETCH_ERROR",
            ),
        }
    }

    pub async fn mark_as_read(pool: &DbPool, notification_id: Uuid, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                NotificationRepository::mark_as_read(tx, notification_id, user_id).await
            })
        })
        .await
        {
            Ok(notification) => formatters::success_response(
                StatusCode::OK,
                notification,
                "NOTIFICATION_MARKED_READ",
            ),
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Notification not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "NOTIFICATION_MARK_READ_ERROR",
                )
            }
        }
    }

    pub async fn mark_all_as_read(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { NotificationRepository::mark_all_as_read(tx, user_id).await })
        })
        .await
        {
            Ok(affected_rows) => {
                let response_data = serde_json::json!({
                    "message": "All notifications marked as read",
                    "affected_count": affected_rows
                });
                formatters::success_response(
                    StatusCode::OK,
                    response_data,
                    "ALL_NOTIFICATIONS_MARKED_READ",
                )
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "ALL_NOTIFICATIONS_MARK_READ_ERROR",
            ),
        }
    }

    pub async fn delete_notification(
        pool: &DbPool,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(
                async move { NotificationRepository::delete(tx, notification_id, user_id).await },
            )
        })
        .await
        {
            Ok(affected_rows) => {
                if affected_rows > 0 {
                    formatters::success_response(
                        StatusCode::OK,
                        "Notification deleted successfully",
                        "NOTIFICATION_DELETED",
                    )
                } else {
                    formatters::error_response(
                        StatusCode::NOT_FOUND,
                        "Notification not found",
                        "NOTIFICATION_NOT_FOUND",
                    )
                }
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "NOTIFICATION_DELETE_ERROR",
            ),
        }
    }

    pub async fn get_unread_count(pool: &DbPool, user_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { NotificationRepository::get_unread_count(tx, user_id).await })
        })
        .await
        {
            Ok(count) => {
                let notification_count = NotificationCount {
                    unread_count: count,
                };
                formatters::success_response(
                    StatusCode::OK,
                    notification_count,
                    "UNREAD_COUNT_FETCHED",
                )
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "UNREAD_COUNT_FETCH_ERROR",
            ),
        }
    }

    pub async fn send_notification(
        pool: &DbPool,
        request: SendNotificationRequest,
    ) -> HttpResponse {
        if request.user_ids.is_empty() {
            return formatters::error_response(
                StatusCode::BAD_REQUEST,
                "User IDs list cannot be empty",
                "INVALID_REQUEST",
            );
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

        match with_transaction(pool, |tx| {
            Box::pin(async move { NotificationRepository::create_bulk(tx, &notifications).await })
        })
        .await
        {
            Ok(created_notifications) => {
                let response_data = serde_json::json!({
                    "message": "Notifications sent successfully",
                    "count": created_notifications.len(),
                    "notifications": created_notifications
                });
                formatters::success_response(
                    StatusCode::CREATED,
                    response_data,
                    "NOTIFICATIONS_SENT",
                )
            }
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "NOTIFICATIONS_SEND_ERROR",
            ),
        }
    }

    pub async fn create_notification(pool: &DbPool, notification: NewNotification) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { NotificationRepository::create(tx, &notification).await })
        })
        .await
        {
            Ok(notification) => formatters::success_response(
                StatusCode::CREATED,
                notification,
                "NOTIFICATION_CREATED",
            ),
            Err(e) => formatters::error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                &e.to_string(),
                "NOTIFICATION_CREATION_ERROR",
            ),
        }
    }
}
