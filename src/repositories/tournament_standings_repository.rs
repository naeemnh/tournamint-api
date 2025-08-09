use chrono::Utc;
use rust_decimal::Decimal;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use sqlx::PgConnection;
use uuid::Uuid;

use crate::models::tournament_standings::{
    EditableTournamentStandings, NewTournamentStandings, ParticipantStats, TournamentStandings,
    TournamentStandingsIden,
};

pub struct TournamentStandingsRepository;

#[allow(dead_code)]
impl TournamentStandingsRepository {
    pub async fn create(
        tx: &mut PgConnection,
        data: NewTournamentStandings,
    ) -> Result<TournamentStandings, sqlx::Error> {
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
                data.tournament_id.into(),
                data.category_id.into(),
                data.participant_id.into(),
                data.participant_name.into(),
                data.participant_type.into(),
                data.points.unwrap_or(Decimal::ZERO).into(),
                data.matches_played.unwrap_or(0).into(),
                data.matches_won.unwrap_or(0).into(),
                data.matches_lost.unwrap_or(0).into(),
                data.matches_drawn.unwrap_or(0).into(),
                data.sets_won.unwrap_or(0).into(),
                data.sets_lost.unwrap_or(0).into(),
                data.games_won.unwrap_or(0).into(),
                data.games_lost.unwrap_or(0).into(),
                data.goal_difference.into(),
                data.bonus_points.into(),
                data.penalty_points.into(),
                Utc::now().into(),
            ])
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn get_by_tournament_id(
        tx: &mut PgConnection,
        tournament_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_by_category_id(
        tx: &mut PgConnection,
        category_id: Uuid,
    ) -> Result<Vec<TournamentStandings>, sqlx::Error> {
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

        sqlx::query_as_with(&sql, values).fetch_all(&mut *tx).await
    }

    pub async fn get_by_participant(
        tx: &mut PgConnection,
        tournament_id: Uuid,
        participant_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<Option<TournamentStandings>, sqlx::Error> {
        let mut query = Query::select();
        query
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
            .and_where(Expr::col(TournamentStandingsIden::ParticipantId).eq(participant_id));

        if let Some(cat_id) = category_id {
            query.and_where(Expr::col(TournamentStandingsIden::CategoryId).eq(cat_id));
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values)
            .fetch_optional(&mut *tx)
            .await
    }

    pub async fn update(
        tx: &mut PgConnection,
        id: Uuid,
        data: EditableTournamentStandings,
    ) -> Result<TournamentStandings, sqlx::Error> {
        let mut query = Query::update();
        query.table(TournamentStandingsIden::Table);

        if let Some(position) = data.position {
            query.value(TournamentStandingsIden::Position, position);
        }
        if let Some(points) = data.points {
            query.value(TournamentStandingsIden::Points, points);
        }
        if let Some(matches_played) = data.matches_played {
            query.value(TournamentStandingsIden::MatchesPlayed, matches_played);
        }
        if let Some(matches_won) = data.matches_won {
            query.value(TournamentStandingsIden::MatchesWon, matches_won);
        }
        if let Some(matches_lost) = data.matches_lost {
            query.value(TournamentStandingsIden::MatchesLost, matches_lost);
        }
        if let Some(matches_drawn) = data.matches_drawn {
            query.value(TournamentStandingsIden::MatchesDrawn, matches_drawn);
        }
        if let Some(sets_won) = data.sets_won {
            query.value(TournamentStandingsIden::SetsWon, sets_won);
        }
        if let Some(sets_lost) = data.sets_lost {
            query.value(TournamentStandingsIden::SetsLost, sets_lost);
        }
        if let Some(games_won) = data.games_won {
            query.value(TournamentStandingsIden::GamesWon, games_won);
        }
        if let Some(games_lost) = data.games_lost {
            query.value(TournamentStandingsIden::GamesLost, games_lost);
        }
        if let Some(goal_difference) = data.goal_difference {
            query.value(TournamentStandingsIden::GoalDifference, goal_difference);
        }
        if let Some(head_to_head) = data.head_to_head {
            query.value(TournamentStandingsIden::HeadToHead, head_to_head);
        }
        if let Some(bonus_points) = data.bonus_points {
            query.value(TournamentStandingsIden::BonusPoints, bonus_points);
        }
        if let Some(penalty_points) = data.penalty_points {
            query.value(TournamentStandingsIden::PenaltyPoints, penalty_points);
        }
        if let Some(is_eliminated) = data.is_eliminated {
            query.value(TournamentStandingsIden::IsEliminated, is_eliminated);
        }
        if let Some(elimination_round) = data.elimination_round {
            query.value(TournamentStandingsIden::EliminationRound, elimination_round);
        }

        query.value(TournamentStandingsIden::LastUpdated, Utc::now());

        let (sql, values) = query
            .and_where(Expr::col(TournamentStandingsIden::Id).eq(id))
            .returning_all()
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_as_with(&sql, values).fetch_one(&mut *tx).await
    }

    pub async fn delete_by_tournament(
        tx: &mut PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<u64, sqlx::Error> {
        let mut query = Query::delete();
        query
            .from_table(TournamentStandingsIden::Table)
            .and_where(Expr::col(TournamentStandingsIden::TournamentId).eq(tournament_id));

        if let Some(cat_id) = category_id {
            query.and_where(Expr::col(TournamentStandingsIden::CategoryId).eq(cat_id));
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(&mut *tx)
            .await
            .map(|result| result.rows_affected())
    }

    pub async fn update_positions(
        tx: &mut PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<u64, sqlx::Error> {
        let mut sort_order = vec!["points DESC".to_string(), "matches_won DESC".to_string()];

        // Add goal difference for sports that use it
        sort_order.push("goal_difference DESC".to_string());
        sort_order.push("games_won - games_lost DESC".to_string());
        sort_order.push("sets_won - sets_lost DESC".to_string());
        sort_order.push("participant_name ASC".to_string());

        let sort_clause = sort_order.join(", ");

        let mut where_clause = format!("tournament_id = '{}'", tournament_id);
        if let Some(cat_id) = category_id {
            where_clause.push_str(&format!(" AND category_id = '{}'", cat_id));
        } else {
            where_clause.push_str(" AND category_id IS NULL");
        }

        let update_sql = format!(
            "UPDATE tournament_standings 
             SET position = ranked.new_position, last_updated = NOW()
             FROM (
                 SELECT id, ROW_NUMBER() OVER (ORDER BY {}) as new_position
                 FROM tournament_standings
                 WHERE {}
             ) as ranked
             WHERE tournament_standings.id = ranked.id",
            sort_clause, where_clause
        );

        sqlx::query(&update_sql)
            .execute(&mut *tx)
            .await
            .map(|result| result.rows_affected())
    }

    pub async fn bulk_upsert(
        tx: &mut PgConnection,
        standings_data: Vec<(Uuid, Option<Uuid>, Uuid, String, String, ParticipantStats)>,
    ) -> Result<u64, sqlx::Error> {
        let mut affected_rows = 0;

        for (
            tournament_id,
            category_id,
            participant_id,
            participant_name,
            participant_type,
            stats,
        ) in standings_data
        {
            let upsert_sql = r#"
                INSERT INTO tournament_standings (
                    tournament_id, category_id, participant_id, participant_name, participant_type,
                    position, points, matches_played, matches_won, matches_lost, matches_drawn,
                    sets_won, sets_lost, games_won, games_lost, goal_difference,
                    is_eliminated, last_updated, created_at
                ) VALUES ($1, $2, $3, $4, $5, 0, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, false, NOW(), NOW())
                ON CONFLICT (tournament_id, COALESCE(category_id, '00000000-0000-0000-0000-000000000000'::uuid), participant_id)
                DO UPDATE SET
                    points = EXCLUDED.points,
                    matches_played = EXCLUDED.matches_played,
                    matches_won = EXCLUDED.matches_won,
                    matches_lost = EXCLUDED.matches_lost,
                    matches_drawn = EXCLUDED.matches_drawn,
                    sets_won = EXCLUDED.sets_won,
                    sets_lost = EXCLUDED.sets_lost,
                    games_won = EXCLUDED.games_won,
                    games_lost = EXCLUDED.games_lost,
                    goal_difference = EXCLUDED.goal_difference,
                    last_updated = NOW()
            "#;

            let points = Decimal::from(stats.matches_won * 3 + stats.matches_drawn); // Standard 3-1-0 point system
            let goal_difference = stats.points_scored - stats.points_conceded;

            let result = sqlx::query(upsert_sql)
                .bind(tournament_id)
                .bind(category_id)
                .bind(participant_id)
                .bind(participant_name)
                .bind(participant_type)
                .bind(points)
                .bind(stats.matches_played)
                .bind(stats.matches_won)
                .bind(stats.matches_lost)
                .bind(stats.matches_drawn)
                .bind(stats.sets_won)
                .bind(stats.sets_lost)
                .bind(stats.games_won)
                .bind(stats.games_lost)
                .bind(goal_difference)
                .execute(&mut *tx)
                .await?;

            affected_rows += result.rows_affected();
        }

        Ok(affected_rows)
    }
}
