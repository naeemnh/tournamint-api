use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::match_domain::{
    EditableMatchResult, MatchResult, MatchResultRepository, MatchScoreSummary, NewMatchResult,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden Definitions ====================

pub enum MatchResultIden {
    Table,
    Id,
    MatchId,
    SetNumber,
    Participant1Score,
    Participant2Score,
    PeriodNumber,
    PeriodName,
    ScoringData,
    Participant1Stats,
    Participant2Stats,
    CreatedAt,
    UpdatedAt,
}

impl Iden for MatchResultIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                MatchResultIden::Table => "match_results",
                MatchResultIden::Id => "id",
                MatchResultIden::MatchId => "match_id",
                MatchResultIden::SetNumber => "set_number",
                MatchResultIden::Participant1Score => "participant1_score",
                MatchResultIden::Participant2Score => "participant2_score",
                MatchResultIden::PeriodNumber => "period_number",
                MatchResultIden::PeriodName => "period_name",
                MatchResultIden::ScoringData => "scoring_data",
                MatchResultIden::Participant1Stats => "participant1_stats",
                MatchResultIden::Participant2Stats => "participant2_stats",
                MatchResultIden::CreatedAt => "created_at",
                MatchResultIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct MatchResultRow {
    id: Uuid,
    match_id: Uuid,
    set_number: Option<i32>,
    participant1_score: Option<i32>,
    participant2_score: Option<i32>,
    period_number: Option<i32>,
    period_name: Option<String>,
    scoring_data: Option<JsonValue>,
    participant1_stats: Option<JsonValue>,
    participant2_stats: Option<JsonValue>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<MatchResultRow> for MatchResult {
    fn from(row: MatchResultRow) -> Self {
        MatchResult {
            id: row.id,
            match_id: row.match_id,
            set_number: row.set_number,
            participant1_score: row.participant1_score,
            participant2_score: row.participant2_score,
            period_number: row.period_number,
            period_name: row.period_name,
            scoring_data: row.scoring_data,
            participant1_stats: row.participant1_stats,
            participant2_stats: row.participant2_stats,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(Debug, FromRow)]
struct MatchScoreSummaryRow {
    match_id: Uuid,
    #[sqlx(rename = "participant1_sets_won!")]
    participant1_sets_won: i64,
    #[sqlx(rename = "participant2_sets_won!")]
    participant2_sets_won: i64,
    #[sqlx(rename = "participant1_total_points!")]
    participant1_total_points: i64,
    #[sqlx(rename = "participant2_total_points!")]
    participant2_total_points: i64,
}

impl From<MatchScoreSummaryRow> for MatchScoreSummary {
    fn from(row: MatchScoreSummaryRow) -> Self {
        MatchScoreSummary {
            match_id: row.match_id,
            participant1_sets_won: row.participant1_sets_won,
            participant2_sets_won: row.participant2_sets_won,
            participant1_total_points: row.participant1_total_points,
            participant2_total_points: row.participant2_total_points,
        }
    }
}

// ==================== Match Result Repository Implementation ====================

pub struct PgMatchResultRepository {
    pool: DbPool,
}

impl PgMatchResultRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MatchResultRepository for PgMatchResultRepository {
    async fn create(&self, new_result: NewMatchResult) -> Result<MatchResult, AppError> {
        let (sql, values) = Query::insert()
            .into_table(MatchResultIden::Table)
            .columns([
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
            ])
            .values_panic([
                new_result.match_id.into(),
                new_result.set_number.into(),
                new_result.participant1_score.into(),
                new_result.participant2_score.into(),
                new_result.period_number.into(),
                new_result.period_name.into(),
                new_result.scoring_data.into(),
                new_result.participant1_stats.into(),
                new_result.participant2_stats.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: MatchResultRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(MatchResult::from(row))
    }

    async fn find_by_id(&self, result_id: Uuid) -> Result<Option<MatchResult>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                MatchResultIden::Id,
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
                MatchResultIden::CreatedAt,
                MatchResultIden::UpdatedAt,
            ])
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::Id).eq(result_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchResultRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchResult::from))
    }

    async fn update(
        &self,
        result_id: Uuid,
        result_data: EditableMatchResult,
    ) -> Result<Option<MatchResult>, AppError> {
        let mut query = Query::update();
        query.table(MatchResultIden::Table);

        if let Some(participant1_score) = result_data.participant1_score {
            query.value(MatchResultIden::Participant1Score, participant1_score);
        }
        if let Some(participant2_score) = result_data.participant2_score {
            query.value(MatchResultIden::Participant2Score, participant2_score);
        }
        if let Some(scoring_data) = result_data.scoring_data {
            query.value(MatchResultIden::ScoringData, scoring_data);
        }
        if let Some(participant1_stats) = result_data.participant1_stats {
            query.value(MatchResultIden::Participant1Stats, participant1_stats);
        }
        if let Some(participant2_stats) = result_data.participant2_stats {
            query.value(MatchResultIden::Participant2Stats, participant2_stats);
        }

        query.value(MatchResultIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(MatchResultIden::Id).eq(result_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchResultRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchResult::from))
    }

    async fn delete(&self, result_id: Uuid) -> Result<Option<MatchResult>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::Id).eq(result_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<MatchResultRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchResult::from))
    }

    async fn find_by_match(&self, match_id: Uuid) -> Result<Vec<MatchResult>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                MatchResultIden::Id,
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
                MatchResultIden::CreatedAt,
                MatchResultIden::UpdatedAt,
            ])
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .order_by(MatchResultIden::SetNumber, sea_query::Order::Asc)
            .order_by(MatchResultIden::PeriodNumber, sea_query::Order::Asc)
            .order_by(MatchResultIden::CreatedAt, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<MatchResultRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(MatchResult::from).collect())
    }

    async fn get_match_score_summary(
        &self,
        match_id: Uuid,
    ) -> Result<Option<MatchScoreSummary>, AppError> {
        let sql = r#"
            SELECT 
                match_id,
                COUNT(CASE WHEN participant1_score > participant2_score THEN 1 END) as "participant1_sets_won!",
                COUNT(CASE WHEN participant2_score > participant1_score THEN 1 END) as "participant2_sets_won!",
                COALESCE(SUM(participant1_score), 0) as "participant1_total_points!",
                COALESCE(SUM(participant2_score), 0) as "participant2_total_points!"
            FROM match_results
            WHERE match_id = $1
            GROUP BY match_id
        "#;

        let row: Option<MatchScoreSummaryRow> = sqlx::query_as(sql)
            .bind(match_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(MatchScoreSummary::from))
    }

    async fn find_by_set(
        &self,
        match_id: Uuid,
        set_number: i32,
    ) -> Result<Vec<MatchResult>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                MatchResultIden::Id,
                MatchResultIden::MatchId,
                MatchResultIden::SetNumber,
                MatchResultIden::Participant1Score,
                MatchResultIden::Participant2Score,
                MatchResultIden::PeriodNumber,
                MatchResultIden::PeriodName,
                MatchResultIden::ScoringData,
                MatchResultIden::Participant1Stats,
                MatchResultIden::Participant2Stats,
                MatchResultIden::CreatedAt,
                MatchResultIden::UpdatedAt,
            ])
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .and_where(Expr::col(MatchResultIden::SetNumber).eq(set_number))
            .order_by(MatchResultIden::PeriodNumber, sea_query::Order::Asc)
            .order_by(MatchResultIden::CreatedAt, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<MatchResultRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(MatchResult::from).collect())
    }

    async fn delete_by_match(&self, match_id: Uuid) -> Result<u64, AppError> {
        let (sql, values) = Query::delete()
            .from_table(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, values)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn count_by_match(&self, match_id: Uuid) -> Result<i64, AppError> {
        let (sql, values) = Query::select()
            .expr(sea_query::Expr::col(MatchResultIden::Id).count())
            .from(MatchResultIden::Table)
            .and_where(Expr::col(MatchResultIden::MatchId).eq(match_id))
            .build_sqlx(PostgresQueryBuilder);

        let result: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(result.0)
    }
}
