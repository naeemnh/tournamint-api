use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde_json::Value as JsonValue;
use sqlx::FromRow;
use std::fmt::Write;
use uuid::Uuid;

use crate::domain::tournament::{
    EditableTournamentCategory, NewTournamentCategory, TournamentCategory,
    TournamentCategoryRepository,
};
use crate::shared::AppError;

use super::pool::DbPool;
use super::tournament_repo::{team_composition_to_string, TeamCompositionDb};

// ==================== Sea-Query Iden ====================

pub enum TournamentCategoryIden {
    Table,
    Id,
    TournamentId,
    Name,
    Description,
    TeamComposition,
    MinParticipants,
    MaxParticipants,
    EntryFee,
    PrizeDistribution,
    Rules,
    Constraints,
    CreatedAt,
    UpdatedAt,
}

impl Iden for TournamentCategoryIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentCategoryIden::Table => "tournament_categories",
                TournamentCategoryIden::Id => "id",
                TournamentCategoryIden::TournamentId => "tournament_id",
                TournamentCategoryIden::Name => "name",
                TournamentCategoryIden::Description => "description",
                TournamentCategoryIden::TeamComposition => "team_composition",
                TournamentCategoryIden::MinParticipants => "min_participants",
                TournamentCategoryIden::MaxParticipants => "max_participants",
                TournamentCategoryIden::EntryFee => "entry_fee",
                TournamentCategoryIden::PrizeDistribution => "prize_distribution",
                TournamentCategoryIden::Rules => "rules",
                TournamentCategoryIden::Constraints => "constraints",
                TournamentCategoryIden::CreatedAt => "created_at",
                TournamentCategoryIden::UpdatedAt => "updated_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TournamentCategoryRow {
    id: Uuid,
    tournament_id: Uuid,
    name: String,
    description: Option<String>,
    team_composition: TeamCompositionDb,
    min_participants: i32,
    max_participants: Option<i32>,
    entry_fee: Option<Decimal>,
    prize_distribution: Option<JsonValue>,
    rules: Option<JsonValue>,
    constraints: Option<JsonValue>,
    created_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl From<TournamentCategoryRow> for TournamentCategory {
    fn from(row: TournamentCategoryRow) -> Self {
        TournamentCategory {
            id: row.id,
            tournament_id: row.tournament_id,
            name: row.name,
            description: row.description,
            team_composition: row.team_composition.into(),
            min_participants: row.min_participants,
            max_participants: row.max_participants,
            entry_fee: row.entry_fee,
            prize_distribution: row.prize_distribution,
            rules: row.rules,
            constraints: row.constraints,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

// ==================== Repository ====================

pub struct PgTournamentCategoryRepository {
    pool: DbPool,
}

impl PgTournamentCategoryRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TournamentCategoryRepository for PgTournamentCategoryRepository {
    async fn create(
        &self,
        new_category: NewTournamentCategory,
    ) -> Result<TournamentCategory, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TournamentCategoryIden::Table)
            .columns([
                TournamentCategoryIden::TournamentId,
                TournamentCategoryIden::Name,
                TournamentCategoryIden::Description,
                TournamentCategoryIden::TeamComposition,
                TournamentCategoryIden::MinParticipants,
                TournamentCategoryIden::MaxParticipants,
                TournamentCategoryIden::EntryFee,
                TournamentCategoryIden::PrizeDistribution,
                TournamentCategoryIden::Rules,
                TournamentCategoryIden::Constraints,
            ])
            .values_panic([
                new_category.tournament_id.into(),
                new_category.name.into(),
                new_category.description.into(),
                team_composition_to_string(new_category.team_composition).into(),
                new_category.min_participants.unwrap_or(2).into(),
                new_category.max_participants.into(),
                new_category.entry_fee.into(),
                new_category.prize_distribution.into(),
                new_category.rules.into(),
                new_category.constraints.into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TournamentCategoryRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(TournamentCategory::from(row))
    }

    async fn get_by_tournament(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentCategory>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentCategoryIden::Id,
                TournamentCategoryIden::TournamentId,
                TournamentCategoryIden::Name,
                TournamentCategoryIden::Description,
                TournamentCategoryIden::TeamComposition,
                TournamentCategoryIden::MinParticipants,
                TournamentCategoryIden::MaxParticipants,
                TournamentCategoryIden::EntryFee,
                TournamentCategoryIden::PrizeDistribution,
                TournamentCategoryIden::Rules,
                TournamentCategoryIden::Constraints,
                TournamentCategoryIden::CreatedAt,
                TournamentCategoryIden::UpdatedAt,
            ])
            .from(TournamentCategoryIden::Table)
            .and_where(Expr::col(TournamentCategoryIden::TournamentId).eq(tournament_id))
            .order_by(TournamentCategoryIden::Name, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentCategoryRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(TournamentCategory::from).collect())
    }

    async fn get_by_id(&self, category_id: Uuid) -> Result<Option<TournamentCategory>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentCategoryIden::Id,
                TournamentCategoryIden::TournamentId,
                TournamentCategoryIden::Name,
                TournamentCategoryIden::Description,
                TournamentCategoryIden::TeamComposition,
                TournamentCategoryIden::MinParticipants,
                TournamentCategoryIden::MaxParticipants,
                TournamentCategoryIden::EntryFee,
                TournamentCategoryIden::PrizeDistribution,
                TournamentCategoryIden::Rules,
                TournamentCategoryIden::Constraints,
                TournamentCategoryIden::CreatedAt,
                TournamentCategoryIden::UpdatedAt,
            ])
            .from(TournamentCategoryIden::Table)
            .and_where(Expr::col(TournamentCategoryIden::Id).eq(category_id))
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentCategoryRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentCategory::from))
    }

    async fn update(
        &self,
        category_id: Uuid,
        category_data: EditableTournamentCategory,
    ) -> Result<Option<TournamentCategory>, AppError> {
        let mut query = Query::update();
        query.table(TournamentCategoryIden::Table);

        if let Some(name) = category_data.name {
            query.value(TournamentCategoryIden::Name, name);
        }
        if let Some(description) = category_data.description {
            query.value(TournamentCategoryIden::Description, description);
        }
        if let Some(team_composition) = category_data.team_composition {
            query.value(
                TournamentCategoryIden::TeamComposition,
                team_composition_to_string(team_composition),
            );
        }
        if let Some(min_participants) = category_data.min_participants {
            query.value(TournamentCategoryIden::MinParticipants, min_participants);
        }
        if let Some(max_participants) = category_data.max_participants {
            query.value(TournamentCategoryIden::MaxParticipants, max_participants);
        }
        if let Some(entry_fee) = category_data.entry_fee {
            query.value(TournamentCategoryIden::EntryFee, entry_fee);
        }
        if let Some(prize_distribution) = category_data.prize_distribution {
            query.value(TournamentCategoryIden::PrizeDistribution, prize_distribution);
        }
        if let Some(rules) = category_data.rules {
            query.value(TournamentCategoryIden::Rules, rules);
        }
        if let Some(constraints) = category_data.constraints {
            query.value(TournamentCategoryIden::Constraints, constraints);
        }

        query.value(TournamentCategoryIden::UpdatedAt, Utc::now());
        query.and_where(Expr::col(TournamentCategoryIden::Id).eq(category_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentCategoryRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentCategory::from))
    }

    async fn delete(&self, category_id: Uuid) -> Result<Option<TournamentCategory>, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TournamentCategoryIden::Table)
            .and_where(Expr::col(TournamentCategoryIden::Id).eq(category_id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentCategoryRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentCategory::from))
    }
}
