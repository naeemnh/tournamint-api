use sqlx::{Pool, Postgres, Row};
use uuid::Uuid;

use crate::models::notification::{NewNotification, Notification};

pub struct NotificationRepository;

impl NotificationRepository {
    pub async fn create(
        pool: &Pool<Postgres>,
        notification: &NewNotification,
    ) -> Result<Notification, sqlx::Error> {
        let query = r#"
            INSERT INTO notifications (user_id, title, message, notification_type, tournament_id, match_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, user_id, title, message, notification_type, is_read, tournament_id, match_id, created_at, updated_at
        "#;

        sqlx::query_as::<_, Notification>(query)
            .bind(notification.user_id)
            .bind(&notification.title)
            .bind(&notification.message)
            .bind(&notification.notification_type)
            .bind(notification.tournament_id)
            .bind(notification.match_id)
            .fetch_one(pool)
            .await
    }

    pub async fn get_by_user_id(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        let query = r#"
            SELECT id, user_id, title, message, notification_type, is_read, tournament_id, match_id, created_at, updated_at
            FROM notifications
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        "#;

        sqlx::query_as::<_, Notification>(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    pub async fn get_unread_by_user_id(
        pool: &Pool<Postgres>,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        let query = r#"
            SELECT id, user_id, title, message, notification_type, is_read, tournament_id, match_id, created_at, updated_at
            FROM notifications
            WHERE user_id = $1 AND is_read = FALSE
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
        "#;

        sqlx::query_as::<_, Notification>(query)
            .bind(user_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await
    }

    pub async fn mark_as_read(
        pool: &Pool<Postgres>,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification, sqlx::Error> {
        let query = r#"
            UPDATE notifications
            SET is_read = TRUE, updated_at = NOW()
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, title, message, notification_type, is_read, tournament_id, match_id, created_at, updated_at
        "#;

        sqlx::query_as::<_, Notification>(query)
            .bind(notification_id)
            .bind(user_id)
            .fetch_one(pool)
            .await
    }

    pub async fn mark_all_as_read(
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let query = r#"
            UPDATE notifications
            SET is_read = TRUE, updated_at = NOW()
            WHERE user_id = $1 AND is_read = FALSE
        "#;

        let result = sqlx::query(query)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn delete(
        pool: &Pool<Postgres>,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let query = "DELETE FROM notifications WHERE id = $1 AND user_id = $2";

        let result = sqlx::query(query)
            .bind(notification_id)
            .bind(user_id)
            .execute(pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_unread_count(
        pool: &Pool<Postgres>,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let query = r#"
            SELECT COUNT(*) as count
            FROM notifications
            WHERE user_id = $1 AND is_read = FALSE
        "#;

        let row = sqlx::query(query)
            .bind(user_id)
            .fetch_one(pool)
            .await?;

        Ok(row.get::<i64, _>("count"))
    }

    pub async fn create_bulk(
        pool: &Pool<Postgres>,
        notifications: &[NewNotification],
    ) -> Result<Vec<Notification>, sqlx::Error> {
        if notifications.is_empty() {
            return Ok(vec![]);
        }

        let mut query_builder = sqlx::QueryBuilder::new(
            "INSERT INTO notifications (user_id, title, message, notification_type, tournament_id, match_id) "
        );

        query_builder.push_values(notifications, |mut b, notification| {
            b.push_bind(notification.user_id)
                .push_bind(&notification.title)
                .push_bind(&notification.message)
                .push_bind(&notification.notification_type)
                .push_bind(notification.tournament_id)
                .push_bind(notification.match_id);
        });

        query_builder.push(" RETURNING id, user_id, title, message, notification_type, is_read, tournament_id, match_id, created_at, updated_at");

        let query = query_builder.build_query_as::<Notification>();
        
        query.fetch_all(pool).await
    }

    pub async fn get_by_id(
        pool: &Pool<Postgres>,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification, sqlx::Error> {
        let query = r#"
            SELECT id, user_id, title, message, notification_type, is_read, tournament_id, match_id, created_at, updated_at
            FROM notifications
            WHERE id = $1 AND user_id = $2
        "#;

        sqlx::query_as::<_, Notification>(query)
            .bind(notification_id)
            .bind(user_id)
            .fetch_one(pool)
            .await
    }
}
