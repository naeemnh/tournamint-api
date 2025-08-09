use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::notification::{NewNotification, Notification, NotificationIden};

pub struct NotificationRepository;

impl NotificationRepository {
    pub async fn create(
        tx: &mut PgConnection,
        notification: &NewNotification,
    ) -> Result<Notification, sqlx::Error> {
        let (sql, values) = Query::insert()
            .into_table(NotificationIden::Table)
            .columns([
                NotificationIden::UserId,
                NotificationIden::Title,
                NotificationIden::Message,
                NotificationIden::NotificationType,
                NotificationIden::TournamentId,
                NotificationIden::MatchId,
            ])
            .values_panic([
                notification.user_id.into(),
                notification.title.clone().into(),
                notification.message.clone().into(),
                format!("{:?}", notification.notification_type)
                    .to_lowercase()
                    .into(),
                notification.tournament_id.into(),
                notification.match_id.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_user_id(
        tx: &mut PgConnection,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                NotificationIden::Id,
                NotificationIden::UserId,
                NotificationIden::Title,
                NotificationIden::Message,
                NotificationIden::NotificationType,
                NotificationIden::IsRead,
                NotificationIden::TournamentId,
                NotificationIden::MatchId,
                NotificationIden::CreatedAt,
                NotificationIden::UpdatedAt,
            ])
            .from(NotificationIden::Table)
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .order_by(NotificationIden::CreatedAt, sea_query::Order::Desc)
            .limit(limit as u64)
            .offset(offset as u64)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_unread_by_user_id(
        tx: &mut PgConnection,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Notification>, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                NotificationIden::Id,
                NotificationIden::UserId,
                NotificationIden::Title,
                NotificationIden::Message,
                NotificationIden::NotificationType,
                NotificationIden::IsRead,
                NotificationIden::TournamentId,
                NotificationIden::MatchId,
                NotificationIden::CreatedAt,
                NotificationIden::UpdatedAt,
            ])
            .from(NotificationIden::Table)
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .and_where(Expr::col(NotificationIden::IsRead).eq(false))
            .order_by(NotificationIden::CreatedAt, sea_query::Order::Desc)
            .limit(limit as u64)
            .offset(offset as u64)
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn mark_as_read(
        tx: &mut PgConnection,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(NotificationIden::Table)
            .values([
                (NotificationIden::IsRead, true.into()),
                (NotificationIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(NotificationIden::Id).eq(notification_id))
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn mark_all_as_read(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let (sql, values) = Query::update()
            .table(NotificationIden::Table)
            .values([
                (NotificationIden::IsRead, true.into()),
                (NotificationIden::UpdatedAt, Utc::now().into()),
            ])
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .and_where(Expr::col(NotificationIden::IsRead).eq(false))
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, values).execute(&mut *tx).await?;

        Ok(result.rows_affected())
    }

    pub async fn delete(
        tx: &mut PgConnection,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<u64, sqlx::Error> {
        let (sql, values) = Query::delete()
            .from_table(NotificationIden::Table)
            .and_where(Expr::col(NotificationIden::Id).eq(notification_id))
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, values).execute(&mut *tx).await?;

        Ok(result.rows_affected())
    }

    pub async fn get_unread_count(
        tx: &mut PgConnection,
        user_id: Uuid,
    ) -> Result<i64, sqlx::Error> {
        let (sql, values) = Query::select()
            .expr(sea_query::Expr::col(NotificationIden::Id).count())
            .from(NotificationIden::Table)
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .and_where(Expr::col(NotificationIden::IsRead).eq(false))
            .build_sqlx(PostgresQueryBuilder);

        let result: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *tx)
            .await?;

        Ok(result.0)
    }

    pub async fn create_bulk(
        tx: &mut PgConnection,
        notifications: &[NewNotification],
    ) -> Result<Vec<Notification>, sqlx::Error> {
        if notifications.is_empty() {
            return Ok(vec![]);
        }

        // Use raw SQL for bulk operations as Sea-Query doesn't handle multiple value sets well
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

        query.fetch_all(&mut *tx).await
    }

    pub async fn get_by_id(
        tx: &mut PgConnection,
        notification_id: Uuid,
        user_id: Uuid,
    ) -> Result<Notification, sqlx::Error> {
        let (sql, values) = Query::select()
            .columns([
                NotificationIden::Id,
                NotificationIden::UserId,
                NotificationIden::Title,
                NotificationIden::Message,
                NotificationIden::NotificationType,
                NotificationIden::IsRead,
                NotificationIden::TournamentId,
                NotificationIden::MatchId,
                NotificationIden::CreatedAt,
                NotificationIden::UpdatedAt,
            ])
            .from(NotificationIden::Table)
            .and_where(Expr::col(NotificationIden::Id).eq(notification_id))
            .and_where(Expr::col(NotificationIden::UserId).eq(user_id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }
}
