use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::notification::{
    NewNotification, Notification, NotificationRepository, NotificationType,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden ====================

enum NotificationIden {
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

// ==================== Enum mapping ====================

fn notification_type_to_db(t: &NotificationType) -> &'static str {
    match t {
        NotificationType::TournamentUpdate => "tournament_update",
        NotificationType::MatchReminder => "match_reminder",
        NotificationType::ResultPosted => "result_posted",
        NotificationType::RegistrationConfirmed => "registration_confirmed",
    }
}

fn notification_type_from_db(s: &str) -> Option<NotificationType> {
    match s {
        "tournament_update" => Some(NotificationType::TournamentUpdate),
        "match_reminder" => Some(NotificationType::MatchReminder),
        "result_posted" => Some(NotificationType::ResultPosted),
        "registration_confirmed" => Some(NotificationType::RegistrationConfirmed),
        _ => None,
    }
}

// ==================== Row type ====================

#[derive(Debug, FromRow)]
struct NotificationRow {
    id: Uuid,
    user_id: Uuid,
    title: String,
    message: String,
    notification_type: String,
    is_read: bool,
    tournament_id: Option<Uuid>,
    match_id: Option<Uuid>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<NotificationRow> for Notification {
    fn from(row: NotificationRow) -> Self {
        Notification {
            id: row.id,
            user_id: row.user_id,
            title: row.title,
            message: row.message,
            notification_type: notification_type_from_db(&row.notification_type)
                .unwrap_or(NotificationType::TournamentUpdate),
            is_read: row.is_read,
            tournament_id: row.tournament_id,
            match_id: row.match_id,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// SELECT list with enum as text for decoding into NotificationRow
const NOTIFICATION_SELECT: &str = "id, user_id, title, message, notification_type::text as notification_type, is_read, tournament_id, match_id, created_at, updated_at";

// ==================== Repository ====================

pub struct PgNotificationRepository {
    pool: DbPool,
}

impl PgNotificationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn create(&self, new_notification: NewNotification) -> Result<Notification, AppError> {
        let sql = r#"
            INSERT INTO notifications (user_id, title, message, notification_type, tournament_id, match_id)
            VALUES ($1, $2, $3, $4::notification_type, $5, $6)
            RETURNING id, user_id, title, message, notification_type::text as notification_type, is_read, tournament_id, match_id, created_at, updated_at
        "#;
        let row: NotificationRow = sqlx::query_as(sql)
            .bind(new_notification.user_id)
            .bind(&new_notification.title)
            .bind(&new_notification.message)
            .bind(notification_type_to_db(&new_notification.notification_type))
            .bind(new_notification.tournament_id)
            .bind(new_notification.match_id)
            .fetch_one(&self.pool)
            .await?;
        Ok(Notification::from(row))
    }

    async fn get_by_id(&self, notification_id: Uuid) -> Result<Option<Notification>, AppError> {
        let sql = format!(
            "SELECT {} FROM notifications WHERE id = $1",
            NOTIFICATION_SELECT
        );
        let row: Option<NotificationRow> = sqlx::query_as(&sql)
            .bind(notification_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Notification::from))
    }

    async fn get_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, AppError> {
        let sql = format!(
            "SELECT {} FROM notifications WHERE user_id = $1 ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            NOTIFICATION_SELECT
        );
        let rows: Vec<NotificationRow> = sqlx::query_as(&sql)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Notification::from).collect())
    }

    async fn get_unread_by_user_id(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, AppError> {
        let sql = format!(
            "SELECT {} FROM notifications WHERE user_id = $1 AND is_read = false ORDER BY created_at DESC LIMIT $2 OFFSET $3",
            NOTIFICATION_SELECT
        );
        let rows: Vec<NotificationRow> = sqlx::query_as(&sql)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(Notification::from).collect())
    }

    async fn mark_as_read(&self, notification_id: Uuid) -> Result<Option<Notification>, AppError> {
        let sql = r#"
            UPDATE notifications SET is_read = true, updated_at = $2
            WHERE id = $1
            RETURNING id, user_id, title, message, notification_type::text as notification_type, is_read, tournament_id, match_id, created_at, updated_at
        "#;
        let row: Option<NotificationRow> = sqlx::query_as(sql)
            .bind(notification_id)
            .bind(Utc::now())
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(Notification::from))
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<u64, AppError> {
        let (sql, values) = Query::update()
            .table(NotificationIden::Table)
            .values([
                (NotificationIden::IsRead, true.into()),
                (NotificationIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .and_where(Expr::col(NotificationIden::IsRead).eq(false))
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, values).execute(&self.pool).await?;
        Ok(result.rows_affected())
    }

    async fn delete(&self, notification_id: Uuid) -> Result<Option<Notification>, AppError> {
        let notification = self.get_by_id(notification_id).await?;
        if notification.is_some() {
            let (sql, values) = Query::delete()
                .from_table(NotificationIden::Table)
                .and_where(Expr::col(NotificationIden::Id).eq(notification_id))
                .build_sqlx(PostgresQueryBuilder);
            sqlx::query_with(&sql, values).execute(&self.pool).await?;
        }
        Ok(notification)
    }

    async fn get_unread_count(&self, user_id: Uuid) -> Result<i64, AppError> {
        let (sql, values) = Query::select()
            .expr(Expr::col(NotificationIden::Id).count())
            .from(NotificationIden::Table)
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .and_where(Expr::col(NotificationIden::IsRead).eq(false))
            .build_sqlx(PostgresQueryBuilder);

        let row: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;
        Ok(row.0)
    }

    async fn create_bulk(
        &self,
        notifications: Vec<NewNotification>,
    ) -> Result<Vec<Notification>, AppError> {
        if notifications.is_empty() {
            return Ok(vec![]);
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO notifications (user_id, title, message, notification_type, tournament_id, match_id) ",
        );
        query_builder.push_values(notifications, |mut b, n| {
            b.push_bind(n.user_id)
                .push_bind(n.title.clone())
                .push_bind(n.message.clone())
                .push_bind(notification_type_to_db(&n.notification_type))
                .push_bind(n.tournament_id)
                .push_bind(n.match_id);
        });
        query_builder.push(
            " RETURNING id, user_id, title, message, notification_type::text as notification_type, is_read, tournament_id, match_id, created_at, updated_at",
        );

        let rows: Vec<NotificationRow> =
            query_builder.build_query_as().fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(Notification::from).collect())
    }
}
