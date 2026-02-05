use std::sync::Arc;
use uuid::Uuid;

use crate::domain::notification::{NewNotification, Notification, NotificationRepository};
use crate::shared::AppError;

/// Notification domain services
pub struct NotificationServices<R>
where
    R: NotificationRepository,
{
    notification_repo: Arc<R>,
}

impl<R> NotificationServices<R>
where
    R: NotificationRepository,
{
    pub fn new(notification_repo: Arc<R>) -> Self {
        Self { notification_repo }
    }

    pub async fn get_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, AppError> {
        self.notification_repo
            .get_by_user_id(user_id, limit, offset)
            .await
    }

    pub async fn get_unread_notifications(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, AppError> {
        self.notification_repo
            .get_unread_by_user_id(user_id, limit, offset)
            .await
    }

    pub async fn mark_as_read(
        &self,
        notification_id: Uuid,
    ) -> Result<Option<Notification>, AppError> {
        self.notification_repo.mark_as_read(notification_id).await
    }

    pub async fn mark_all_as_read(&self, user_id: Uuid) -> Result<u64, AppError> {
        self.notification_repo.mark_all_as_read(user_id).await
    }

    pub async fn delete_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<Option<Notification>, AppError> {
        self.notification_repo.delete(notification_id).await
    }

    pub async fn get_unread_count(&self, user_id: Uuid) -> Result<i64, AppError> {
        self.notification_repo.get_unread_count(user_id).await
    }

    pub async fn send_notification(&self, data: NewNotification) -> Result<Notification, AppError> {
        self.notification_repo.create(data).await
    }

    pub async fn send_bulk_notifications(
        &self,
        notifications: Vec<NewNotification>,
    ) -> Result<Vec<Notification>, AppError> {
        self.notification_repo.create_bulk(notifications).await
    }

    pub async fn get_notification(
        &self,
        notification_id: Uuid,
    ) -> Result<Option<Notification>, AppError> {
        self.notification_repo.get_by_id(notification_id).await
    }
}
