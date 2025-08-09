use std::fmt::Write;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "notification_type", rename_all = "snake_case")]
pub enum NotificationType {
    TournamentUpdate,
    MatchReminder,
    ResultPosted,
    RegistrationConfirmed,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Notification {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub is_read: bool,
    pub tournament_id: Option<Uuid>,
    pub match_id: Option<Uuid>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewNotification {
    pub user_id: Uuid,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub tournament_id: Option<Uuid>,
    pub match_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationCount {
    pub unread_count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MarkAllReadRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendNotificationRequest {
    pub user_ids: Vec<Uuid>,
    pub title: String,
    pub message: String,
    pub notification_type: NotificationType,
    pub tournament_id: Option<Uuid>,
    pub match_id: Option<Uuid>,
}

pub enum NotificationIden {
    Table,
    Id,
    UserId,
    Title,
    Message,
    NotificationType,
    IsRead,
    TournamentId,
    MatchId,
    CreatedAt,
    UpdatedAt,
}

impl Iden for NotificationIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                NotificationIden::Table => "notifications",
                NotificationIden::Id => "id",
                NotificationIden::UserId => "user_id",
                NotificationIden::Title => "title",
                NotificationIden::Message => "message",
                NotificationIden::NotificationType => "notification_type",
                NotificationIden::IsRead => "is_read",
                NotificationIden::TournamentId => "tournament_id",
                NotificationIden::MatchId => "match_id",
                NotificationIden::CreatedAt => "created_at",
                NotificationIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}
