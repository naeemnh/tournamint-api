use async_trait::async_trait;
use uuid::Uuid;

use super::entity::Notification;
use super::value_objects::NewNotification;
use crate::shared::AppError;

/// Repository trait for Notification entity operations
#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, new_notification: NewNotification) -> Result<Notification, AppError>;
    async fn get_by_user_id(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Notification>, AppError>;
    async fn get_unread_by_user_id(&self, user_id: Uuid, limit: i64, offset: i64) -> Result<Vec<Notification>, AppError>;
    async fn mark_as_read(&self, notification_id: Uuid) -> Result<Option<Notification>, AppError>;
    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<u64, AppError>;
    async fn delete(&self, notification_id: Uuid) -> Result<Option<Notification>, AppError>;
    async fn get_unread_count(&self, user_id: Uuid) -> Result<i64, AppError>;
    async fn create_bulk(&self, notifications: Vec<NewNotification>) -> Result<Vec<Notification>, AppError>;
    async fn get_by_id(&self, notification_id: Uuid) -> Result<Option<Notification>, AppError>;
}
