use async_trait::async_trait;
use chrono::Utc;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::tournament::{
    BracketStatus, EditableTournamentBracket, NewTournamentBracket, TournamentBracket,
    TournamentBracketRepository,
};
use crate::shared::AppError;

use super::pool::DbPool;
use super::tournament_repo::{bracket_status_to_string, bracket_type_to_string, BracketStatusDb, BracketTypeDb};

// ==================== Sea-Query Iden ====================

pub enum TournamentBracketIden {
    Table,
    Id,
    TournamentId,
    CategoryId,
    BracketType,
    Status,
    TotalRounds,
    CurrentRound,
    BracketData,
    Settings,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentBracketIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentBracketIden::Table => "tournament_brackets",
                TournamentBracketIden::Id => "id",
                TournamentBracketIden::TournamentId => "tournament_id",
                TournamentBracketIden::CategoryId => "category_id",
                TournamentBracketIden::BracketType => "bracket_type",
                TournamentBracketIden::Status => "status",
                TournamentBracketIden::TotalRounds => "total_rounds",
                TournamentBracketIden::CurrentRound => "current_round",
                TournamentBracketIden::BracketData => "bracket_data",
                TournamentBracketIden::Settings => "settings",
                TournamentBracketIden::CreatedAt => "created_at",
                TournamentBracketIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TournamentBracketRow {
    id: Uuid,
    tournament_id: Uuid,
    category_id: Option<Uuid>,
    bracket_type: BracketTypeDb,
    status: BracketStatusDb,
    total_rounds: i32,
    current_round: i32,
    bracket_data: Option<JsonValue>,
    settings: Option<JsonValue>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<TournamentBracketRow> for TournamentBracket {
    fn from(row: TournamentBracketRow) -> Self {
        TournamentBracket {
            id: row.id,
            tournament_id: row.tournament_id,
            category_id: row.category_id,
            bracket_type: row.bracket_type.into(),
            status: row.status.into(),
            total_rounds: row.total_rounds,
            current_round: row.current_round,
            bracket_data: row.bracket_data,
            settings: row.settings,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ==================== Repository ====================

pub struct PgTournamentBracketRepository {
    pool: DbPool,
}

impl PgTournamentBracketRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TournamentBracketRepository for PgTournamentBracketRepository {
    async fn create(
        &self,
        new_bracket: NewTournamentBracket,
    ) -> Result<TournamentBracket, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TournamentBracketIden::Table)
            .columns([
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
            ])
            .values_panic([
                new_bracket.tournament_id.into(),
                new_bracket.category_id.into(),
                bracket_type_to_string(new_bracket.bracket_type).into(),
                new_bracket.total_rounds.into(),
                new_bracket.bracket_data.into(),
                new_bracket.settings.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TournamentBracketRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(TournamentBracket::from(row))
    }

    async fn get_by_tournament_id(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentBracket>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentBracketIden::Id,
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::Status,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::CurrentRound,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
                TournamentBracketIden::CreatedAt,
                TournamentBracketIden::UpdatedAt,
            ])
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::TournamentId).eq(tournament_id))
            .order_by(TournamentBracketIden::CreatedAt, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentBracketRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(TournamentBracket::from).collect())
    }

    async fn get_by_category_id(
        &self,
        category_id: Uuid,
    ) -> Result<Option<TournamentBracket>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentBracketIden::Id,
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::Status,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::CurrentRound,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
                TournamentBracketIden::CreatedAt,
                TournamentBracketIden::UpdatedAt,
            ])
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::CategoryId).eq(category_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentBracketRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentBracket::from))
    }

    async fn get_by_id(&self, bracket_id: Uuid) -> Result<Option<TournamentBracket>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentBracketIden::Id,
                TournamentBracketIden::TournamentId,
                TournamentBracketIden::CategoryId,
                TournamentBracketIden::BracketType,
                TournamentBracketIden::Status,
                TournamentBracketIden::TotalRounds,
                TournamentBracketIden::CurrentRound,
                TournamentBracketIden::BracketData,
                TournamentBracketIden::Settings,
                TournamentBracketIden::CreatedAt,
                TournamentBracketIden::UpdatedAt,
            ])
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::Id).eq(bracket_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentBracketRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentBracket::from))
    }

    async fn update(
        &self,
        bracket_id: Uuid,
        bracket_data: EditableTournamentBracket,
    ) -> Result<Option<TournamentBracket>, AppError> {
        let mut query = Query::update();
        query.table(TournamentBracketIden::Table);

        if let Some(status) = bracket_data.status {
            query.value(
                TournamentBracketIden::Status,
                bracket_status_to_string(status),
            );
        }
        if let Some(current_round) = bracket_data.current_round {
            query.value(TournamentBracketIden::CurrentRound, current_round);
        }
        if let Some(bracket_data_json) = bracket_data.bracket_data {
            query.value(TournamentBracketIden::BracketData, bracket_data_json);
        }
        if let Some(settings) = bracket_data.settings {
            query.value(TournamentBracketIden::Settings, settings);
        }

        query.value(TournamentBracketIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(TournamentBracketIden::Id).eq(bracket_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentBracketRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentBracket::from))
    }

    async fn update_status(
        &self,
        bracket_id: Uuid,
        status: BracketStatus,
    ) -> Result<Option<TournamentBracket>, AppError> {
        let (sql, values) = Query::update()
            .table(TournamentBracketIden::Table)
            .value(
                TournamentBracketIden::Status,
                bracket_status_to_string(status),
            )
            .value(TournamentBracketIden::UpdatedAt, Utc::now())
            .and_where(Expr::col(TournamentBracketIden::Id).eq(bracket_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentBracketRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentBracket::from))
    }

    async fn delete(&self, bracket_id: Uuid) -> Result<Option<TournamentBracket>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::Id).eq(bracket_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentBracketRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentBracket::from))
    }

    async fn exists_for_tournament(&self, tournament_id: Uuid) -> Result<bool, AppError> {
        let (sql, values) = Query::select()
            .expr(Expr::cust("COUNT(*)"))
            .from(TournamentBracketIden::Table)
            .and_where(Expr::col(TournamentBracketIden::TournamentId).eq(tournament_id))
            .build_sqlx(PostgresQueryBuilder);

        let count: (i64,) = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(count.0 > 0)
    }
}
