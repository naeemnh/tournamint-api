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
    EditableTournamentStandings, NewTournamentStandings, TournamentStandings,
    TournamentStandingsRepository,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Sea-Query Iden ====================

pub enum TournamentStandingsIden {
    Table,
    Id,
    TournamentId,
    CategoryId,
    ParticipantId,
    ParticipantName,
    ParticipantType,
    Position,
    Points,
    MatchesPlayed,
    MatchesWon,
    MatchesLost,
    MatchesDrawn,
    SetsWon,
    SetsLost,
    GamesWon,
    GamesLost,
    GoalDifference,
    HeadToHead,
    BonusPoints,
    PenaltyPoints,
    IsEliminated,
    EliminationRound,
    LastUpdated,
    CreatedAt,
}

impl Iden for TournamentStandingsIden {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}",
            match self {
                TournamentStandingsIden::Table => "tournament_standings",
                TournamentStandingsIden::Id => "id",
                TournamentStandingsIden::TournamentId => "tournament_id",
                TournamentStandingsIden::CategoryId => "category_id",
                TournamentStandingsIden::ParticipantId => "participant_id",
                TournamentStandingsIden::ParticipantName => "participant_name",
                TournamentStandingsIden::ParticipantType => "participant_type",
                TournamentStandingsIden::Position => "position",
                TournamentStandingsIden::Points => "points",
                TournamentStandingsIden::MatchesPlayed => "matches_played",
                TournamentStandingsIden::MatchesWon => "matches_won",
                TournamentStandingsIden::MatchesLost => "matches_lost",
                TournamentStandingsIden::MatchesDrawn => "matches_drawn",
                TournamentStandingsIden::SetsWon => "sets_won",
                TournamentStandingsIden::SetsLost => "sets_lost",
                TournamentStandingsIden::GamesWon => "games_won",
                TournamentStandingsIden::GamesLost => "games_lost",
                TournamentStandingsIden::GoalDifference => "goal_difference",
                TournamentStandingsIden::HeadToHead => "head_to_head",
                TournamentStandingsIden::BonusPoints => "bonus_points",
                TournamentStandingsIden::PenaltyPoints => "penalty_points",
                TournamentStandingsIden::IsEliminated => "is_eliminated",
                TournamentStandingsIden::EliminationRound => "elimination_round",
                TournamentStandingsIden::LastUpdated => "last_updated",
                TournamentStandingsIden::CreatedAt => "created_at",
            }
        )
        .unwrap()
    }
}

// ==================== Row Types ====================

#[derive(Debug, FromRow)]
struct TournamentStandingsRow {
    id: Uuid,
    tournament_id: Uuid,
    category_id: Option<Uuid>,
    participant_id: Uuid,
    participant_name: String,
    participant_type: String,
    position: i32,
    points: Decimal,
    matches_played: i32,
    matches_won: i32,
    matches_lost: i32,
    matches_drawn: i32,
    sets_won: i32,
    sets_lost: i32,
    games_won: i32,
    games_lost: i32,
    goal_difference: Option<i32>,
    head_to_head: Option<JsonValue>,
    bonus_points: Option<Decimal>,
    penalty_points: Option<Decimal>,
    is_eliminated: bool,
    elimination_round: Option<String>,
    last_updated: chrono::DateTime<Utc>,
    created_at: chrono::DateTime<Utc>,
}

impl From<TournamentStandingsRow> for TournamentStandings {
    fn from(row: TournamentStandingsRow) -> Self {
        TournamentStandings {
            id: row.id,
            tournament_id: row.tournament_id,
            category_id: row.category_id,
            participant_id: row.participant_id,
            participant_name: row.participant_name,
            participant_type: row.participant_type,
            position: row.position,
            points: row.points,
            matches_played: row.matches_played,
            matches_won: row.matches_won,
            matches_lost: row.matches_lost,
            matches_drawn: row.matches_drawn,
            sets_won: row.sets_won,
            sets_lost: row.sets_lost,
            games_won: row.games_won,
            games_lost: row.games_lost,
            goal_difference: row.goal_difference,
            head_to_head: row.head_to_head,
            bonus_points: row.bonus_points,
            penalty_points: row.penalty_points,
            is_eliminated: row.is_eliminated,
            elimination_round: row.elimination_round,
            last_updated: row.last_updated,
            created_at: row.created_at,
        }
    }
}

// ==================== Repository ====================

pub struct PgTournamentStandingsRepository {
    pool: DbPool,
}

impl PgTournamentStandingsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TournamentStandingsRepository for PgTournamentStandingsRepository {
    async fn create(
        &self,
        new_standings: NewTournamentStandings,
    ) -> Result<TournamentStandings, AppError> {
        let (sql, values) = Query::insert()
            .into_table(TournamentStandingsIden::Table)
            .columns([
                TournamentStandingsIden::TournamentId,
                TournamentStandingsIden::CategoryId,
                TournamentStandingsIden::ParticipantId,
                TournamentStandingsIden::ParticipantName,
                TournamentStandingsIden::ParticipantType,
                TournamentStandingsIden::Points,
                TournamentStandingsIden::MatchesPlayed,
                TournamentStandingsIden::MatchesWon,
                TournamentStandingsIden::MatchesLost,
                TournamentStandingsIden::MatchesDrawn,
                TournamentStandingsIden::SetsWon,
                TournamentStandingsIden::SetsLost,
                TournamentStandingsIden::GamesWon,
                TournamentStandingsIden::GamesLost,
                TournamentStandingsIden::GoalDifference,
                TournamentStandingsIden::BonusPoints,
                TournamentStandingsIden::PenaltyPoints,
                TournamentStandingsIden::LastUpdated,
            ])
            .values_panic([
                new_standings.tournament_id.into(),
                new_standings.category_id.into(),
                new_standings.participant_id.into(),
                new_standings.participant_name.into(),
                new_standings.participant_type.into(),
                new_standings.points.unwrap_or(Decimal::ZERO).into(),
                new_standings.matches_played.unwrap_or(0).into(),
                new_standings.matches_won.unwrap_or(0).into(),
                new_standings.matches_lost.unwrap_or(0).into(),
                new_standings.matches_drawn.unwrap_or(0).into(),
                new_standings.sets_won.unwrap_or(0).into(),
                new_standings.sets_lost.unwrap_or(0).into(),
                new_standings.games_won.unwrap_or(0).into(),
                new_standings.games_lost.unwrap_or(0).into(),
                new_standings.goal_difference.into(),
                new_standings.bonus_points.into(),
                new_standings.penalty_points.into(),
                Utc::now().into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        let row: TournamentStandingsRow = sqlx::query_as_with(&sql, values)
            .fetch_one(&self.pool)
            .await?;

        Ok(TournamentStandings::from(row))
    }

    async fn get_by_tournament_id(
        &self,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentStandingsIden::Id,
                TournamentStandingsIden::TournamentId,
                TournamentStandingsIden::CategoryId,
                TournamentStandingsIden::ParticipantId,
                TournamentStandingsIden::ParticipantName,
                TournamentStandingsIden::ParticipantType,
                TournamentStandingsIden::Position,
                TournamentStandingsIden::Points,
                TournamentStandingsIden::MatchesPlayed,
                TournamentStandingsIden::MatchesWon,
                TournamentStandingsIden::MatchesLost,
                TournamentStandingsIden::MatchesDrawn,
                TournamentStandingsIden::SetsWon,
                TournamentStandingsIden::SetsLost,
                TournamentStandingsIden::GamesWon,
                TournamentStandingsIden::GamesLost,
                TournamentStandingsIden::GoalDifference,
                TournamentStandingsIden::HeadToHead,
                TournamentStandingsIden::BonusPoints,
                TournamentStandingsIden::PenaltyPoints,
                TournamentStandingsIden::IsEliminated,
                TournamentStandingsIden::EliminationRound,
                TournamentStandingsIden::LastUpdated,
                TournamentStandingsIden::CreatedAt,
            ])
            .from(TournamentStandingsIden::Table)
            .and_where(Expr::col(TournamentStandingsIden::TournamentId).eq(tournament_id))
            .order_by(TournamentStandingsIden::Position, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentStandingsRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(TournamentStandings::from).collect())
    }

    async fn get_by_category_id(
        &self,
        category_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentStandingsIden::Id,
                TournamentStandingsIden::TournamentId,
                TournamentStandingsIden::CategoryId,
                TournamentStandingsIden::ParticipantId,
                TournamentStandingsIden::ParticipantName,
                TournamentStandingsIden::ParticipantType,
                TournamentStandingsIden::Position,
                TournamentStandingsIden::Points,
                TournamentStandingsIden::MatchesPlayed,
                TournamentStandingsIden::MatchesWon,
                TournamentStandingsIden::MatchesLost,
                TournamentStandingsIden::MatchesDrawn,
                TournamentStandingsIden::SetsWon,
                TournamentStandingsIden::SetsLost,
                TournamentStandingsIden::GamesWon,
                TournamentStandingsIden::GamesLost,
                TournamentStandingsIden::GoalDifference,
                TournamentStandingsIden::HeadToHead,
                TournamentStandingsIden::BonusPoints,
                TournamentStandingsIden::PenaltyPoints,
                TournamentStandingsIden::IsEliminated,
                TournamentStandingsIden::EliminationRound,
                TournamentStandingsIden::LastUpdated,
                TournamentStandingsIden::CreatedAt,
            ])
            .from(TournamentStandingsIden::Table)
            .and_where(Expr::col(TournamentStandingsIden::CategoryId).eq(category_id))
            .order_by(TournamentStandingsIden::Position, sea_query::Order::Asc)
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentStandingsRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(TournamentStandings::from).collect())
    }

    async fn get_by_participant(
        &self,
        participant_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, AppError> {
        let (sql, values) = Query::select()
            .columns([
                TournamentStandingsIden::Id,
                TournamentStandingsIden::TournamentId,
                TournamentStandingsIden::CategoryId,
                TournamentStandingsIden::ParticipantId,
                TournamentStandingsIden::ParticipantName,
                TournamentStandingsIden::ParticipantType,
                TournamentStandingsIden::Position,
                TournamentStandingsIden::Points,
                TournamentStandingsIden::MatchesPlayed,
                TournamentStandingsIden::MatchesWon,
                TournamentStandingsIden::MatchesLost,
                TournamentStandingsIden::MatchesDrawn,
                TournamentStandingsIden::SetsWon,
                TournamentStandingsIden::SetsLost,
                TournamentStandingsIden::GamesWon,
                TournamentStandingsIden::GamesLost,
                TournamentStandingsIden::GoalDifference,
                TournamentStandingsIden::HeadToHead,
                TournamentStandingsIden::BonusPoints,
                TournamentStandingsIden::PenaltyPoints,
                TournamentStandingsIden::IsEliminated,
                TournamentStandingsIden::EliminationRound,
                TournamentStandingsIden::LastUpdated,
                TournamentStandingsIden::CreatedAt,
            ])
            .from(TournamentStandingsIden::Table)
            .and_where(Expr::col(TournamentStandingsIden::ParticipantId).eq(participant_id))
            .build_sqlx(PostgresQueryBuilder);

        let rows: Vec<TournamentStandingsRow> = sqlx::query_as_with(&sql, values)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows.into_iter().map(TournamentStandings::from).collect())
    }

    async fn update(
        &self,
        standings_id: Uuid,
        standings_data: EditableTournamentStandings,
    ) -> Result<Option<TournamentStandings>, AppError> {
        let mut query = Query::update();
        query.table(TournamentStandingsIden::Table);

        if let Some(position) = standings_data.position {
            query.value(TournamentStandingsIden::Position, position);
        }
        if let Some(points) = standings_data.points {
            query.value(TournamentStandingsIden::Points, points);
        }
        if let Some(matches_played) = standings_data.matches_played {
            query.value(TournamentStandingsIden::MatchesPlayed, matches_played);
        }
        if let Some(matches_won) = standings_data.matches_won {
            query.value(TournamentStandingsIden::MatchesWon, matches_won);
        }
        if let Some(matches_lost) = standings_data.matches_lost {
            query.value(TournamentStandingsIden::MatchesLost, matches_lost);
        }
        if let Some(matches_drawn) = standings_data.matches_drawn {
            query.value(TournamentStandingsIden::MatchesDrawn, matches_drawn);
        }
        if let Some(sets_won) = standings_data.sets_won {
            query.value(TournamentStandingsIden::SetsWon, sets_won);
        }
        if let Some(sets_lost) = standings_data.sets_lost {
            query.value(TournamentStandingsIden::SetsLost, sets_lost);
        }
        if let Some(games_won) = standings_data.games_won {
            query.value(TournamentStandingsIden::GamesWon, games_won);
        }
        if let Some(games_lost) = standings_data.games_lost {
            query.value(TournamentStandingsIden::GamesLost, games_lost);
        }
        if let Some(goal_difference) = standings_data.goal_difference {
            query.value(TournamentStandingsIden::GoalDifference, goal_difference);
        }
        if let Some(head_to_head) = standings_data.head_to_head {
            query.value(TournamentStandingsIden::HeadToHead, head_to_head);
        }
        if let Some(bonus_points) = standings_data.bonus_points {
            query.value(TournamentStandingsIden::BonusPoints, bonus_points);
        }
        if let Some(penalty_points) = standings_data.penalty_points {
            query.value(TournamentStandingsIden::PenaltyPoints, penalty_points);
        }
        if let Some(is_eliminated) = standings_data.is_eliminated {
            query.value(TournamentStandingsIden::IsEliminated, is_eliminated);
        }
        if let Some(elimination_round) = standings_data.elimination_round {
            query.value(TournamentStandingsIden::EliminationRound, elimination_round);
        }

        query.value(TournamentStandingsIden::LastUpdated, Utc::now());
        query.and_where(Expr::col(TournamentStandingsIden::Id).eq(standings_id));
        query.returning_all();

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let row: Option<TournamentStandingsRow> = sqlx::query_as_with(&sql, values)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(TournamentStandings::from))
    }

    async fn delete_by_tournament(&self, tournament_id: Uuid) -> Result<u64, AppError> {
        let (sql, values) = Query::delete()
            .from_table(TournamentStandingsIden::Table)
            .and_where(Expr::col(TournamentStandingsIden::TournamentId).eq(tournament_id))
            .build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_with(&sql, values)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    async fn bulk_upsert(
        &self,
        standings: Vec<NewTournamentStandings>,
    ) -> Result<Vec<TournamentStandings>, AppError> {
        let mut results = Vec::new();

        for standing in standings {
            // Try to find existing standing
            let existing = sqlx::query_as::<_, TournamentStandingsRow>(
                r#"
                SELECT * FROM tournament_standings 
                WHERE tournament_id = $1 
                AND COALESCE(category_id, '00000000-0000-0000-0000-000000000000'::uuid) = COALESCE($2, '00000000-0000-0000-0000-000000000000'::uuid)
                AND participant_id = $3
                "#,
            )
            .bind(standing.tournament_id)
            .bind(standing.category_id)
            .bind(standing.participant_id)
            .fetch_optional(&self.pool)
            .await?;

            if let Some(existing_row) = existing {
                // Update existing
                let updated = self
                    .update(
                        existing_row.id,
                        EditableTournamentStandings {
                            position: None,
                            points: standing.points,
                            matches_played: standing.matches_played,
                            matches_won: standing.matches_won,
                            matches_lost: standing.matches_lost,
                            matches_drawn: standing.matches_drawn,
                            sets_won: standing.sets_won,
                            sets_lost: standing.sets_lost,
                            games_won: standing.games_won,
                            games_lost: standing.games_lost,
                            goal_difference: standing.goal_difference,
                            head_to_head: None,
                            bonus_points: standing.bonus_points,
                            penalty_points: standing.penalty_points,
                            is_eliminated: None,
                            elimination_round: None,
                        },
                    )
                    .await?;
                if let Some(s) = updated {
                    results.push(s);
                }
            } else {
                // Create new
                let created = self.create(standing).await?;
                results.push(created);
            }
        }

        Ok(results)
    }
}
