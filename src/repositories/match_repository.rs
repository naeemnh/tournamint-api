use anyhow::Result;
use chrono::Utc;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::match_model::{
    CreateMatchRequest, Match, MatchIden, MatchStatus, UpdateMatchRequest, UpdateMatchStatusRequest,
};

pub struct MatchRepository {
    pool: PgPool,
}

impl MatchRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, request: CreateMatchRequest) -> Result<Match> {
        let mut conn = self.pool.acquire().await?;
        let id = Uuid::new_v4();

        let (sql, values) = Query::insert()
            .into_table(MatchIden::Table)
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
            ])
            .values_panic([
                id.into(),
                request.tournament_category_id.into(),
                request.participant1_team_id.into(),
                request.participant1_player_id.into(),
                request.participant1_partner_id.into(),
                request.participant2_team_id.into(),
                request.participant2_player_id.into(),
                request.participant2_partner_id.into(),
                format!("{:?}", request.match_type).to_lowercase().into(),
                format!("{:?}", request.match_status).to_lowercase().into(),
                request.round_number.into(),
                request.match_number.into(),
                request.scheduled_date.into(),
                request.venue.into(),
                request.court_number.into(),
                request.referee_name.into(),
                request.umpire_name.into(),
                request.notes.into(),
                request.metadata.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let match_data = sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *conn)
            .await?;

        Ok(match_data)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Match>> {
        let mut conn = self.pool.acquire().await?;

        let (sql, values) = Query::select()
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::ActualStartDate,
                MatchIden::ActualEndDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::WinnerParticipant,
                MatchIden::IsDraw,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
                MatchIden::CreatedAt,
                MatchIden::UpdatedAt,
            ])
            .from(MatchIden::Table)
            .and_where(Expr::col(MatchIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        let match_data = sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *conn)
            .await?;

        Ok(match_data)
    }

    pub async fn update(&self, id: Uuid, request: UpdateMatchRequest) -> Result<Match> {
        let mut conn = self.pool.acquire().await?;

        let mut query = Query::update();
        query.table(MatchIden::Table);

        if let Some(scheduled_date) = request.scheduled_date {
            query.value(MatchIden::ScheduledDate, scheduled_date);
        }
        if let Some(venue) = request.venue {
            query.value(MatchIden::Venue, venue);
        }
        if let Some(court_number) = request.court_number {
            query.value(MatchIden::CourtNumber, court_number);
        }
        if let Some(referee_name) = request.referee_name {
            query.value(MatchIden::RefereeName, referee_name);
        }
        if let Some(umpire_name) = request.umpire_name {
            query.value(MatchIden::UmpireName, umpire_name);
        }
        if let Some(notes) = request.notes {
            query.value(MatchIden::Notes, notes);
        }
        if let Some(metadata) = request.metadata {
            query.value(MatchIden::Metadata, metadata);
        }

        query.value(MatchIden::UpdatedAt, sea_query::Value::from(Utc::now()));
        query.and_where(Expr::col(MatchIden::Id).eq(id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let match_data = sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *conn)
            .await?;

        Ok(match_data)
    }

    pub async fn update_status(
        &self,
        id: Uuid,
        request: UpdateMatchStatusRequest,
    ) -> Result<Match> {
        let mut conn = self.pool.acquire().await?;

        let mut query = Query::update();
        query.table(MatchIden::Table);
        query.value(
            MatchIden::MatchStatus,
            sea_query::Value::from(format!("{:?}", request.status).to_lowercase()),
        );

        // Set actual_start_date when match starts
        if matches!(request.status, MatchStatus::InProgress) {
            query.value(
                MatchIden::ActualStartDate,
                sea_query::Value::from(Utc::now()),
            );
        }

        // Set actual_end_date when match ends
        if matches!(
            request.status,
            MatchStatus::Completed | MatchStatus::Cancelled | MatchStatus::Forfeited
        ) {
            query.value(MatchIden::ActualEndDate, sea_query::Value::from(Utc::now()));
        }

        if let Some(winner) = request.winner_participant {
            query.value(MatchIden::WinnerParticipant, sea_query::Value::from(winner));
        }

        if let Some(is_draw) = request.is_draw {
            query.value(MatchIden::IsDraw, sea_query::Value::from(is_draw));
        }

        query.value(MatchIden::UpdatedAt, sea_query::Value::from(Utc::now()));
        query.and_where(Expr::col(MatchIden::Id).eq(id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let match_data = sqlx::query_as_with(&sql, values)
            .fetch_one(&mut *conn)
            .await?;

        Ok(match_data)
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        let (sql, values) = Query::delete()
            .from_table(MatchIden::Table)
            .and_where(Expr::col(MatchIden::Id).eq(id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(&mut *conn).await?;

        Ok(())
    }

    pub async fn find_by_tournament(&self, tournament_id: Uuid) -> Result<Vec<Match>> {
        // For complex joins, we'll use raw SQL as Sea-Query join support is limited
        let sql = r#"
            SELECT 
                m.id, m.tournament_category_id,
                m.participant1_team_id, m.participant1_player_id, m.participant1_partner_id,
                m.participant2_team_id, m.participant2_player_id, m.participant2_partner_id,
                m.match_type, m.match_status,
                m.round_number, m.match_number, m.scheduled_date, m.actual_start_date, m.actual_end_date,
                m.venue, m.court_number, m.winner_participant, m.is_draw,
                m.referee_name, m.umpire_name, m.notes, m.metadata, m.created_at, m.updated_at
            FROM matches m
            JOIN tournament_categories tc ON m.tournament_category_id = tc.id
            WHERE tc.tournament_id = $1
            ORDER BY m.scheduled_date, m.round_number, m.match_number
        "#;

        let matches = sqlx::query_as(sql)
            .bind(tournament_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(matches)
    }

    pub async fn find_by_category(&self, category_id: Uuid) -> Result<Vec<Match>> {
        let mut conn = self.pool.acquire().await?;

        let (sql, values) = Query::select()
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::ActualStartDate,
                MatchIden::ActualEndDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::WinnerParticipant,
                MatchIden::IsDraw,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
                MatchIden::CreatedAt,
                MatchIden::UpdatedAt,
            ])
            .from(MatchIden::Table)
            .and_where(Expr::col(MatchIden::TournamentCategoryId).eq(category_id))
            .order_by(MatchIden::RoundNumber, sea_query::Order::Asc)
            .order_by(MatchIden::MatchNumber, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let matches = sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *conn)
            .await?;

        Ok(matches)
    }

    pub async fn find_scheduled(&self) -> Result<Vec<Match>> {
        let mut conn = self.pool.acquire().await?;

        let (sql, values) = Query::select()
            .columns([
                MatchIden::Id,
                MatchIden::TournamentCategoryId,
                MatchIden::Participant1TeamId,
                MatchIden::Participant1PlayerId,
                MatchIden::Participant1PartnerId,
                MatchIden::Participant2TeamId,
                MatchIden::Participant2PlayerId,
                MatchIden::Participant2PartnerId,
                MatchIden::MatchType,
                MatchIden::MatchStatus,
                MatchIden::RoundNumber,
                MatchIden::MatchNumber,
                MatchIden::ScheduledDate,
                MatchIden::ActualStartDate,
                MatchIden::ActualEndDate,
                MatchIden::Venue,
                MatchIden::CourtNumber,
                MatchIden::WinnerParticipant,
                MatchIden::IsDraw,
                MatchIden::RefereeName,
                MatchIden::UmpireName,
                MatchIden::Notes,
                MatchIden::Metadata,
                MatchIden::CreatedAt,
                MatchIden::UpdatedAt,
            ])
            .from(MatchIden::Table)
            .and_where(Expr::col(MatchIden::MatchStatus).eq("scheduled"))
            .and_where(Expr::col(MatchIden::ScheduledDate).is_not_null())
            .order_by(MatchIden::ScheduledDate, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let matches = sqlx::query_as_with(&sql, values)
            .fetch_all(&mut *conn)
            .await?;

        Ok(matches)
    }
}
