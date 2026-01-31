use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    TournamentUpdate,
    MatchReminder,
    ResultPosted,
    RegistrationConfirmed,
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
