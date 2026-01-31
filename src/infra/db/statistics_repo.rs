use async_trait::async_trait;
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::FromRow;
use uuid::Uuid;

use crate::domain::statistics::{
    AnalyticsDashboard, GameRecord, GrowthMetrics, LeaderboardEntry, PlayerStatistics,
    StatisticsFilters, StatisticsRepository, TeamStatistics, TournamentStatistics,
};
use crate::shared::AppError;

use super::pool::DbPool;

// ==================== Row types (domain entities match DB shape for query_as) ====================

#[derive(Debug, FromRow)]
struct PlayerStatisticsRow {
    player_id: Uuid,
    player_name: String,
    total_tournaments: i64,
    tournaments_won: i64,
    tournaments_runner_up: i64,
    total_matches: i64,
    matches_won: i64,
    matches_lost: i64,
    win_rate: Decimal,
    total_earnings: Decimal,
    average_placement: Decimal,
    best_placement: i32,
    current_ranking: Option<i32>,
    ranking_points: Decimal,
    last_active: chrono::DateTime<Utc>,
}

impl From<PlayerStatisticsRow> for PlayerStatistics {
    fn from(row: PlayerStatisticsRow) -> Self {
        PlayerStatistics {
            player_id: row.player_id,
            player_name: row.player_name,
            total_tournaments: row.total_tournaments,
            tournaments_won: row.tournaments_won,
            tournaments_runner_up: row.tournaments_runner_up,
            total_matches: row.total_matches,
            matches_won: row.matches_won,
            matches_lost: row.matches_lost,
            win_rate: row.win_rate,
            total_earnings: row.total_earnings,
            average_placement: row.average_placement,
            best_placement: row.best_placement,
            current_ranking: row.current_ranking,
            ranking_points: row.ranking_points,
            last_active: row.last_active,
        }
    }
}

#[derive(Debug, FromRow)]
struct TeamStatisticsRow {
    team_id: Uuid,
    team_name: String,
    total_tournaments: i64,
    tournaments_won: i64,
    tournaments_runner_up: i64,
    total_matches: i64,
    matches_won: i64,
    matches_lost: i64,
    win_rate: Decimal,
    total_earnings: Decimal,
    average_placement: Decimal,
    best_placement: i32,
    current_ranking: Option<i32>,
    ranking_points: Decimal,
    members_count: i64,
    last_active: chrono::DateTime<Utc>,
}

impl From<TeamStatisticsRow> for TeamStatistics {
    fn from(row: TeamStatisticsRow) -> Self {
        TeamStatistics {
            team_id: row.team_id,
            team_name: row.team_name,
            total_tournaments: row.total_tournaments,
            tournaments_won: row.tournaments_won,
            tournaments_runner_up: row.tournaments_runner_up,
            total_matches: row.total_matches,
            matches_won: row.matches_won,
            matches_lost: row.matches_lost,
            win_rate: row.win_rate,
            total_earnings: row.total_earnings,
            average_placement: row.average_placement,
            best_placement: row.best_placement,
            current_ranking: row.current_ranking,
            ranking_points: row.ranking_points,
            members_count: row.members_count,
            last_active: row.last_active,
        }
    }
}

#[derive(Debug, FromRow)]
struct TournamentStatisticsRow {
    tournament_id: Uuid,
    tournament_name: String,
    total_participants: i64,
    total_teams: i64,
    total_matches: i64,
    completed_matches: i64,
    pending_matches: i64,
    total_prize_pool: Decimal,
    total_registrations: i64,
    completion_rate: Decimal,
    average_match_duration: Option<i32>,
    most_wins_player: Option<String>,
    most_wins_team: Option<String>,
    start_date: chrono::DateTime<Utc>,
    end_date: Option<chrono::DateTime<Utc>>,
}

impl From<TournamentStatisticsRow> for TournamentStatistics {
    fn from(row: TournamentStatisticsRow) -> Self {
        TournamentStatistics {
            tournament_id: row.tournament_id,
            tournament_name: row.tournament_name,
            total_participants: row.total_participants,
            total_teams: row.total_teams,
            total_matches: row.total_matches,
            completed_matches: row.completed_matches,
            pending_matches: row.pending_matches,
            total_prize_pool: row.total_prize_pool,
            total_registrations: row.total_registrations,
            completion_rate: row.completion_rate,
            average_match_duration: row.average_match_duration,
            most_wins_player: row.most_wins_player,
            most_wins_team: row.most_wins_team,
            start_date: row.start_date,
            end_date: row.end_date,
        }
    }
}

#[derive(Debug, FromRow)]
struct LeaderboardEntryRow {
    rank: i64,
    id: Uuid,
    name: String,
    points: Decimal,
    tournaments_won: i64,
    win_rate: Decimal,
    total_earnings: Decimal,
    last_active: chrono::DateTime<Utc>,
}

impl From<LeaderboardEntryRow> for LeaderboardEntry {
    fn from(row: LeaderboardEntryRow) -> Self {
        LeaderboardEntry {
            rank: row.rank,
            id: row.id,
            name: row.name,
            points: row.points,
            tournaments_won: row.tournaments_won,
            win_rate: row.win_rate,
            total_earnings: row.total_earnings,
            last_active: row.last_active,
        }
    }
}

#[derive(Debug, FromRow)]
struct GameRecordRow {
    id: Uuid,
    category: String,
    record_type: String,
    holder_id: Uuid,
    holder_name: String,
    value: Decimal,
    description: String,
    achieved_date: chrono::DateTime<Utc>,
    tournament_id: Option<Uuid>,
    tournament_name: Option<String>,
}

impl From<GameRecordRow> for GameRecord {
    fn from(row: GameRecordRow) -> Self {
        GameRecord {
            id: row.id,
            category: row.category,
            record_type: row.record_type,
            holder_id: row.holder_id,
            holder_name: row.holder_name,
            value: row.value,
            description: row.description,
            achieved_date: row.achieved_date,
            tournament_id: row.tournament_id,
            tournament_name: row.tournament_name,
        }
    }
}

#[derive(Debug, FromRow)]
struct GrowthMetricsRow {
    new_players_this_month: i64,
    new_teams_this_month: i64,
    tournaments_this_month: i64,
    matches_this_month: i64,
    revenue_this_month: Decimal,
    player_growth_rate: Decimal,
    tournament_growth_rate: Decimal,
}

impl From<GrowthMetricsRow> for GrowthMetrics {
    fn from(row: GrowthMetricsRow) -> Self {
        GrowthMetrics {
            new_players_this_month: row.new_players_this_month,
            new_teams_this_month: row.new_teams_this_month,
            tournaments_this_month: row.tournaments_this_month,
            matches_this_month: row.matches_this_month,
            revenue_this_month: row.revenue_this_month,
            player_growth_rate: row.player_growth_rate,
            tournament_growth_rate: row.tournament_growth_rate,
        }
    }
}

// ==================== Repository ====================

pub struct PgStatisticsRepository {
    pool: DbPool,
}

impl PgStatisticsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StatisticsRepository for PgStatisticsRepository {
    async fn get_player_statistics(
        &self,
        player_id: Uuid,
        _filters: Option<StatisticsFilters>,
    ) -> Result<Option<PlayerStatistics>, AppError> {
        let sql = r#"
            WITH player_info AS (
                SELECT p.id AS player_id, p.name AS player_name
                FROM players p
                WHERE p.id = $1
            ),
            tournament_stats AS (
                SELECT
                    COUNT(DISTINCT tc.tournament_id) AS total_tournaments,
                    0::BIGINT AS tournaments_won,
                    0::BIGINT AS tournaments_runner_up,
                    COALESCE(SUM(tr.payment_amount), 0) AS total_earnings
                FROM tournament_registrations tr
                JOIN tournament_categories tc ON tr.tournament_category_id = tc.id
                WHERE tr.player_id = $1 AND tr.registration_status = 'approved'
            ),
            match_stats AS (
                SELECT
                    COUNT(*) AS total_matches,
                    COUNT(CASE WHEN (m.winner_participant = 1 AND m.participant1_player_id = $1)
                                   OR (m.winner_participant = 2 AND m.participant2_player_id = $1) THEN 1 END) AS matches_won,
                    COUNT(CASE WHEN m.match_status = 'completed' AND m.winner_participant IS NOT NULL
                                   AND ((m.winner_participant = 2 AND m.participant1_player_id = $1)
                                        OR (m.winner_participant = 1 AND m.participant2_player_id = $1)) THEN 1 END) AS matches_lost
                FROM matches m
                WHERE (m.participant1_player_id = $1 OR m.participant2_player_id = $1)
                  AND m.match_status = 'completed'
            )
            SELECT
                pi.player_id,
                pi.player_name,
                COALESCE(ts.total_tournaments, 0)::BIGINT AS total_tournaments,
                COALESCE(ts.tournaments_won, 0)::BIGINT AS tournaments_won,
                COALESCE(ts.tournaments_runner_up, 0)::BIGINT AS tournaments_runner_up,
                COALESCE(ms.total_matches, 0)::BIGINT AS total_matches,
                COALESCE(ms.matches_won, 0)::BIGINT AS matches_won,
                COALESCE(ms.matches_lost, 0)::BIGINT AS matches_lost,
                CASE WHEN COALESCE(ms.total_matches, 0) = 0 THEN 0
                     ELSE ROUND(CAST(ms.matches_won AS DECIMAL) / ms.total_matches * 100, 2) END AS win_rate,
                COALESCE(ts.total_earnings, 0) AS total_earnings,
                0::DECIMAL AS average_placement,
                0::INTEGER AS best_placement,
                NULL::INTEGER AS current_ranking,
                (COALESCE(ts.tournaments_won, 0) * 100 + COALESCE(ms.matches_won, 0) * 10)::DECIMAL AS ranking_points,
                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = $1), NOW()) AS last_active
            FROM player_info pi
            LEFT JOIN tournament_stats ts ON true
            LEFT JOIN match_stats ms ON true
        "#;
        let row: Option<PlayerStatisticsRow> = sqlx::query_as(sql)
            .bind(player_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(PlayerStatistics::from))
    }

    async fn get_team_statistics(
        &self,
        team_id: Uuid,
        _filters: Option<StatisticsFilters>,
    ) -> Result<Option<TeamStatistics>, AppError> {
        let sql = r#"
            WITH team_info AS (
                SELECT t.id AS team_id, t.name AS team_name
                FROM teams t
                WHERE t.id = $1
            ),
            tournament_stats AS (
                SELECT
                    COUNT(DISTINCT tc.tournament_id) AS total_tournaments,
                    0::BIGINT AS tournaments_won,
                    0::BIGINT AS tournaments_runner_up,
                    COALESCE(SUM(tr.payment_amount), 0) AS total_earnings
                FROM tournament_registrations tr
                JOIN tournament_categories tc ON tr.tournament_category_id = tc.id
                WHERE tr.team_id = $1 AND tr.registration_status = 'approved'
            ),
            match_stats AS (
                SELECT
                    COUNT(*) AS total_matches,
                    COUNT(CASE WHEN (m.winner_participant = 1 AND m.participant1_team_id = $1)
                                   OR (m.winner_participant = 2 AND m.participant2_team_id = $1) THEN 1 END) AS matches_won,
                    COUNT(CASE WHEN m.match_status = 'completed' AND m.winner_participant IS NOT NULL
                                   AND ((m.winner_participant = 2 AND m.participant1_team_id = $1)
                                        OR (m.winner_participant = 1 AND m.participant2_team_id = $1)) THEN 1 END) AS matches_lost
                FROM matches m
                WHERE (m.participant1_team_id = $1 OR m.participant2_team_id = $1)
                  AND m.match_status = 'completed'
            ),
            member_count AS (
                SELECT COUNT(*) AS members_count FROM team_members WHERE team_id = $1
            )
            SELECT
                ti.team_id,
                ti.team_name,
                COALESCE(ts.total_tournaments, 0)::BIGINT AS total_tournaments,
                COALESCE(ts.tournaments_won, 0)::BIGINT AS tournaments_won,
                COALESCE(ts.tournaments_runner_up, 0)::BIGINT AS tournaments_runner_up,
                COALESCE(ms.total_matches, 0)::BIGINT AS total_matches,
                COALESCE(ms.matches_won, 0)::BIGINT AS matches_won,
                COALESCE(ms.matches_lost, 0)::BIGINT AS matches_lost,
                CASE WHEN COALESCE(ms.total_matches, 0) = 0 THEN 0
                     ELSE ROUND(CAST(ms.matches_won AS DECIMAL) / ms.total_matches * 100, 2) END AS win_rate,
                COALESCE(ts.total_earnings, 0) AS total_earnings,
                0::DECIMAL AS average_placement,
                0::INTEGER AS best_placement,
                NULL::INTEGER AS current_ranking,
                (COALESCE(ts.tournaments_won, 0) * 100 + COALESCE(ms.matches_won, 0) * 10)::DECIMAL AS ranking_points,
                COALESCE(mc.members_count, 0)::BIGINT AS members_count,
                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.team_id = $1), NOW()) AS last_active
            FROM team_info ti
            LEFT JOIN tournament_stats ts ON true
            LEFT JOIN match_stats ms ON true
            LEFT JOIN member_count mc ON true
        "#;
        let row: Option<TeamStatisticsRow> = sqlx::query_as(sql)
            .bind(team_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(TeamStatistics::from))
    }

    async fn get_tournament_statistics(
        &self,
        tournament_id: Uuid,
    ) -> Result<Option<TournamentStatistics>, AppError> {
        let sql = r#"
            WITH tournament_info AS (
                SELECT t.id, t.name, t.start_date, t.end_date, t.prize_pool
                FROM tournaments t
                WHERE t.id = $1
            ),
            registration_stats AS (
                SELECT
                    COUNT(*) AS total_registrations,
                    COUNT(CASE WHEN tr.player_id IS NOT NULL THEN 1 END) AS total_participants,
                    COUNT(CASE WHEN tr.team_id IS NOT NULL THEN 1 END) AS total_teams
                FROM tournament_registrations tr
                JOIN tournament_categories tc ON tr.tournament_category_id = tc.id
                WHERE tc.tournament_id = $1 AND tr.registration_status = 'approved'
            ),
            match_stats AS (
                SELECT
                    COUNT(*) AS total_matches,
                    COUNT(CASE WHEN m.match_status = 'completed' THEN 1 END) AS completed_matches,
                    COUNT(CASE WHEN m.match_status IN ('scheduled', 'in_progress') THEN 1 END) AS pending_matches
                FROM matches m
                JOIN tournament_categories tc ON m.tournament_category_id = tc.id
                WHERE tc.tournament_id = $1
            )
            SELECT
                ti.id AS tournament_id,
                ti.name AS tournament_name,
                COALESCE(rs.total_participants, 0)::BIGINT AS total_participants,
                COALESCE(rs.total_teams, 0)::BIGINT AS total_teams,
                COALESCE(ms.total_matches, 0)::BIGINT AS total_matches,
                COALESCE(ms.completed_matches, 0)::BIGINT AS completed_matches,
                COALESCE(ms.pending_matches, 0)::BIGINT AS pending_matches,
                COALESCE(ti.prize_pool, 0) AS total_prize_pool,
                COALESCE(rs.total_registrations, 0)::BIGINT AS total_registrations,
                CASE WHEN COALESCE(ms.total_matches, 0) = 0 THEN 0
                     ELSE ROUND(CAST(ms.completed_matches AS DECIMAL) / ms.total_matches * 100, 2) END AS completion_rate,
                NULL::INTEGER AS average_match_duration,
                NULL::VARCHAR AS most_wins_player,
                NULL::VARCHAR AS most_wins_team,
                ti.start_date,
                ti.end_date
            FROM tournament_info ti
            LEFT JOIN registration_stats rs ON true
            LEFT JOIN match_stats ms ON true
        "#;
        let row: Option<TournamentStatisticsRow> = sqlx::query_as(sql)
            .bind(tournament_id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(row.map(TournamentStatistics::from))
    }

    async fn get_leaderboard(
        &self,
        category: &str,
        entity_type: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LeaderboardEntry>, AppError> {
        let limit = limit.min(100).max(1);
        let sql = match entity_type {
            "player" => match category {
                "points" => r#"
                    WITH player_stats AS (
                        SELECT p.id, p.name,
                            COALESCE(
                                (SELECT COUNT(DISTINCT tc.tournament_id) FROM tournament_registrations tr
                                 JOIN tournament_categories tc ON tr.tournament_category_id = tc.id
                                 WHERE tr.player_id = p.id AND tr.registration_status = 'approved') * 50 +
                                (SELECT COUNT(*) FROM matches m
                                 WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id)
                                   AND m.match_status = 'completed'
                                   AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id)
                                        OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) * 10,
                                0
                            ) AS ranking_points,
                            (SELECT COUNT(*) FROM matches m
                             WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                               AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id)
                                    OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS matches_won,
                            CASE WHEN (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') = 0 THEN 0
                                 ELSE ROUND(CAST((SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                                    AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS DECIMAL) /
                                    (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') * 100, 2) END AS win_rate,
                            COALESCE((SELECT SUM(tr.payment_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.registration_status = 'approved'), 0) AS total_earnings,
                            COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) AS last_active
                        FROM players p
                    )
                    SELECT ROW_NUMBER() OVER (ORDER BY ranking_points DESC, matches_won DESC) AS rank, id, name,
                           ranking_points AS points, matches_won AS tournaments_won, win_rate, total_earnings, last_active
                    FROM player_stats WHERE ranking_points > 0 ORDER BY ranking_points DESC, matches_won DESC LIMIT $1 OFFSET $2
                "#,
                "wins" => r#"
                    WITH player_stats AS (
                        SELECT p.id, p.name,
                            (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                               AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS matches_won,
                            CASE WHEN (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') = 0 THEN 0
                                 ELSE ROUND(CAST((SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                                    AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS DECIMAL) /
                                    (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') * 100, 2) END AS win_rate,
                            COALESCE((SELECT SUM(tr.payment_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.registration_status = 'approved'), 0) AS total_earnings,
                            COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) AS last_active
                        FROM players p
                    )
                    SELECT ROW_NUMBER() OVER (ORDER BY matches_won DESC, win_rate DESC) AS rank, id, name,
                           matches_won::DECIMAL AS points, matches_won AS tournaments_won, win_rate, total_earnings, last_active
                    FROM player_stats WHERE matches_won > 0 ORDER BY matches_won DESC, win_rate DESC LIMIT $1 OFFSET $2
                "#,
                "earnings" => r#"
                    WITH player_stats AS (
                        SELECT p.id, p.name,
                            COALESCE((SELECT SUM(tr.payment_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.registration_status = 'approved'), 0) AS total_earnings,
                            (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                               AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS matches_won,
                            CASE WHEN (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') = 0 THEN 0
                                 ELSE ROUND(CAST((SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                                    AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS DECIMAL) /
                                    (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') * 100, 2) END AS win_rate,
                            COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) AS last_active
                        FROM players p
                    )
                    SELECT ROW_NUMBER() OVER (ORDER BY total_earnings DESC, matches_won DESC) AS rank, id, name,
                           total_earnings AS points, matches_won AS tournaments_won, win_rate, total_earnings, last_active
                    FROM player_stats WHERE total_earnings > 0 ORDER BY total_earnings DESC, matches_won DESC LIMIT $1 OFFSET $2
                "#,
                _ => r#"
                    WITH player_stats AS (
                        SELECT p.id, p.name,
                            CASE WHEN (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') = 0 THEN 0
                                 ELSE ROUND(CAST((SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                                    AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS DECIMAL) /
                                    (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed') * 100, 2) END AS win_rate,
                            (SELECT COUNT(*) FROM matches m WHERE (m.participant1_player_id = p.id OR m.participant2_player_id = p.id) AND m.match_status = 'completed'
                               AND ((m.winner_participant = 1 AND m.participant1_player_id = p.id) OR (m.winner_participant = 2 AND m.participant2_player_id = p.id))) AS matches_won,
                            COALESCE((SELECT SUM(tr.payment_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.registration_status = 'approved'), 0) AS total_earnings,
                            COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) AS last_active
                        FROM players p
                    )
                    SELECT ROW_NUMBER() OVER (ORDER BY win_rate DESC, matches_won DESC) AS rank, id, name,
                           win_rate AS points, matches_won AS tournaments_won, win_rate, total_earnings, last_active
                    FROM player_stats WHERE matches_won > 0 ORDER BY win_rate DESC, matches_won DESC LIMIT $1 OFFSET $2
                "#,
            },
            _ => match category {
                "points" => r#"
                    WITH team_stats AS (
                        SELECT t.id, t.name,
                            COALESCE(
                                (SELECT COUNT(DISTINCT tc.tournament_id) FROM tournament_registrations tr JOIN tournament_categories tc ON tr.tournament_category_id = tc.id WHERE tr.team_id = t.id AND tr.registration_status = 'approved') * 50 +
                                (SELECT COUNT(*) FROM matches m WHERE (m.participant1_team_id = t.id OR m.participant2_team_id = t.id) AND m.match_status = 'completed'
                                  AND ((m.winner_participant = 1 AND m.participant1_team_id = t.id) OR (m.winner_participant = 2 AND m.participant2_team_id = t.id))) * 10,
                                0
                            ) AS ranking_points,
                            (SELECT COUNT(*) FROM matches m WHERE (m.participant1_team_id = t.id OR m.participant2_team_id = t.id) AND m.match_status = 'completed'
                               AND ((m.winner_participant = 1 AND m.participant1_team_id = t.id) OR (m.winner_participant = 2 AND m.participant2_team_id = t.id))) AS tournaments_won,
                            CASE WHEN (SELECT COUNT(*) FROM matches m WHERE (m.participant1_team_id = t.id OR m.participant2_team_id = t.id) AND m.match_status = 'completed') = 0 THEN 0
                                 ELSE ROUND(CAST((SELECT COUNT(*) FROM matches m WHERE (m.participant1_team_id = t.id OR m.participant2_team_id = t.id) AND m.match_status = 'completed'
                                    AND ((m.winner_participant = 1 AND m.participant1_team_id = t.id) OR (m.winner_participant = 2 AND m.participant2_team_id = t.id))) AS DECIMAL) /
                                    (SELECT COUNT(*) FROM matches m WHERE (m.participant1_team_id = t.id OR m.participant2_team_id = t.id) AND m.match_status = 'completed') * 100, 2) END AS win_rate,
                            COALESCE((SELECT SUM(tr.payment_amount) FROM tournament_registrations tr WHERE tr.team_id = t.id AND tr.registration_status = 'approved'), 0) AS total_earnings,
                            COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.team_id = t.id), t.created_at) AS last_active
                        FROM teams t
                    )
                    SELECT ROW_NUMBER() OVER (ORDER BY ranking_points DESC, tournaments_won DESC) AS rank, id, name,
                           ranking_points AS points, tournaments_won, win_rate, total_earnings, last_active
                    FROM team_stats WHERE ranking_points > 0 ORDER BY ranking_points DESC, tournaments_won DESC LIMIT $1 OFFSET $2
                "#,
                _ => r#"
                    SELECT 1::BIGINT AS rank, gen_random_uuid() AS id, 'Sample Team' AS name, 0::DECIMAL AS points,
                           0::BIGINT AS tournaments_won, 0::DECIMAL AS win_rate, 0::DECIMAL AS total_earnings, NOW() AS last_active
                    WHERE FALSE LIMIT $1 OFFSET $2
                "#,
            },
        };

        let rows: Vec<LeaderboardEntryRow> = sqlx::query_as(sql)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(LeaderboardEntry::from).collect())
    }

    async fn get_game_records(&self, limit: i64) -> Result<Vec<GameRecord>, AppError> {
        let limit = limit.min(100).max(1);
        let sql = r#"
            WITH ranked AS (
                SELECT p.id, p.name, COUNT(*)::DECIMAL AS value, MAX(tr.updated_at) AS achieved_date,
                       'most_tournament_participations' AS category, 'Most tournament participations' AS description
                FROM players p
                JOIN tournament_registrations tr ON p.id = tr.player_id AND tr.registration_status = 'approved'
                JOIN tournament_categories tc ON tr.tournament_category_id = tc.id
                GROUP BY p.id, p.name
                HAVING COUNT(*) > 0
            )
            SELECT uuid_generate_v4() AS id, category, 'player' AS record_type, id AS holder_id, name AS holder_name,
                   value, description, achieved_date, NULL::UUID AS tournament_id, NULL::VARCHAR AS tournament_name
            FROM ranked ORDER BY value DESC LIMIT $1
        "#;
        let rows: Vec<GameRecordRow> = sqlx::query_as(sql)
            .bind(limit)
            .fetch_all(&self.pool)
            .await?;
        Ok(rows.into_iter().map(GameRecord::from).collect())
    }

    async fn get_growth_metrics(&self) -> Result<GrowthMetrics, AppError> {
        let sql = r#"
            WITH current_month AS (
                SELECT
                    COUNT(CASE WHEN DATE_TRUNC('month', p.created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN 1 END) AS new_players_this_month,
                    COUNT(CASE WHEN DATE_TRUNC('month', t.created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN 1 END) AS new_teams_this_month
                FROM players p FULL OUTER JOIN teams t ON FALSE
            ),
            tournament_metrics AS (
                SELECT
                    COUNT(CASE WHEN DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN 1 END) AS tournaments_this_month,
                    0::BIGINT AS matches_this_month,
                    COALESCE(SUM(CASE WHEN DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN prize_pool ELSE 0 END), 0) AS revenue_this_month
                FROM tournaments
            ),
            previous_month AS (
                SELECT
                    COUNT(CASE WHEN DATE_TRUNC('month', p.created_at) = DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month') THEN 1 END) AS prev_players,
                    COUNT(CASE WHEN DATE_TRUNC('month', t.created_at) = DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month') THEN 1 END) AS prev_tournaments
                FROM players p FULL OUTER JOIN tournaments t ON FALSE
            )
            SELECT cm.new_players_this_month, cm.new_teams_this_month, tm.tournaments_this_month, tm.matches_this_month, tm.revenue_this_month,
                   CASE WHEN pm.prev_players = 0 THEN 0 ELSE ROUND(((cm.new_players_this_month::DECIMAL - pm.prev_players) / pm.prev_players * 100), 2) END AS player_growth_rate,
                   CASE WHEN pm.prev_tournaments = 0 THEN 0 ELSE ROUND(((tm.tournaments_this_month::DECIMAL - pm.prev_tournaments) / pm.prev_tournaments * 100), 2) END AS tournament_growth_rate
            FROM current_month cm CROSS JOIN tournament_metrics tm CROSS JOIN previous_month pm
        "#;
        let row: GrowthMetricsRow = sqlx::query_as(sql).fetch_one(&self.pool).await?;
        Ok(GrowthMetrics::from(row))
    }

    async fn get_analytics_dashboard(&self) -> Result<AnalyticsDashboard, AppError> {
        #[derive(Debug, FromRow)]
        struct BasicStatsRow {
            total_players: i64,
            total_teams: i64,
            total_tournaments: i64,
            active_tournaments: i64,
            total_matches: i64,
            total_earnings_distributed: Decimal,
            average_tournament_size: Option<Decimal>,
            most_popular_sport: Option<String>,
        }
        let basic_sql = r#"
            SELECT
                (SELECT COUNT(*) FROM players)::BIGINT AS total_players,
                (SELECT COUNT(*) FROM teams)::BIGINT AS total_teams,
                (SELECT COUNT(*) FROM tournaments)::BIGINT AS total_tournaments,
                (SELECT COUNT(*) FROM tournaments WHERE status IN ('in_progress', 'registration_open'))::BIGINT AS active_tournaments,
                (SELECT COUNT(*) FROM matches)::BIGINT AS total_matches,
                COALESCE((SELECT SUM(prize_pool) FROM tournaments WHERE status = 'completed'), 0) AS total_earnings_distributed,
                CASE WHEN (SELECT COUNT(*) FROM tournaments) = 0 THEN 0
                     ELSE (SELECT AVG(cnt) FROM (SELECT COUNT(*) AS cnt FROM tournament_registrations tr JOIN tournament_categories tc ON tr.tournament_category_id = tc.id WHERE tr.registration_status = 'approved' GROUP BY tc.tournament_id) x) END AS average_tournament_size,
                (SELECT sport_type::TEXT FROM tournaments GROUP BY sport_type ORDER BY COUNT(*) DESC LIMIT 1) AS most_popular_sport
        "#;
        let basic_row: BasicStatsRow = sqlx::query_as(basic_sql).fetch_one(&self.pool).await?;

        let growth_metrics = self.get_growth_metrics().await?;
        let top_players = self.get_leaderboard("points", "player", 5, 0).await?;
        let top_teams = self.get_leaderboard("points", "team", 5, 0).await?;

        let recent_sql = r#"
            SELECT t.id AS tournament_id, t.name AS tournament_name,
                (SELECT COUNT(*) FROM tournament_registrations tr JOIN tournament_categories tc ON tr.tournament_category_id = tc.id WHERE tc.tournament_id = t.id AND tr.registration_status = 'approved' AND tr.player_id IS NOT NULL)::BIGINT AS total_participants,
                (SELECT COUNT(*) FROM tournament_registrations tr JOIN tournament_categories tc ON tr.tournament_category_id = tc.id WHERE tc.tournament_id = t.id AND tr.registration_status = 'approved' AND tr.team_id IS NOT NULL)::BIGINT AS total_teams,
                (SELECT COUNT(*) FROM matches m JOIN tournament_categories tc ON m.tournament_category_id = tc.id WHERE tc.tournament_id = t.id)::BIGINT AS total_matches,
                (SELECT COUNT(*) FROM matches m JOIN tournament_categories tc ON m.tournament_category_id = tc.id WHERE tc.tournament_id = t.id AND m.match_status = 'completed')::BIGINT AS completed_matches,
                (SELECT COUNT(*) FROM matches m JOIN tournament_categories tc ON m.tournament_category_id = tc.id WHERE tc.tournament_id = t.id AND m.match_status IN ('scheduled', 'in_progress'))::BIGINT AS pending_matches,
                COALESCE(t.prize_pool, 0) AS total_prize_pool,
                (SELECT COUNT(*) FROM tournament_registrations tr JOIN tournament_categories tc ON tr.tournament_category_id = tc.id WHERE tc.tournament_id = t.id)::BIGINT AS total_registrations,
                CASE WHEN (SELECT COUNT(*) FROM matches m JOIN tournament_categories tc ON m.tournament_category_id = tc.id WHERE tc.tournament_id = t.id) = 0 THEN 0
                     ELSE ROUND(CAST((SELECT COUNT(*) FROM matches m JOIN tournament_categories tc ON m.tournament_category_id = tc.id WHERE tc.tournament_id = t.id AND m.match_status = 'completed') AS DECIMAL) / (SELECT COUNT(*) FROM matches m JOIN tournament_categories tc ON m.tournament_category_id = tc.id WHERE tc.tournament_id = t.id) * 100, 2) END AS completion_rate,
                NULL::INTEGER AS average_match_duration, NULL::VARCHAR AS most_wins_player, NULL::VARCHAR AS most_wins_team,
                t.start_date, t.end_date
            FROM tournaments t ORDER BY t.created_at DESC LIMIT 5
        "#;
        let recent_tournaments: Vec<TournamentStatisticsRow> =
            sqlx::query_as(recent_sql).fetch_all(&self.pool).await?;

        Ok(AnalyticsDashboard {
            total_players: basic_row.total_players,
            total_teams: basic_row.total_teams,
            total_tournaments: basic_row.total_tournaments,
            active_tournaments: basic_row.active_tournaments,
            total_matches: basic_row.total_matches,
            total_earnings_distributed: basic_row.total_earnings_distributed,
            average_tournament_size: basic_row.average_tournament_size.unwrap_or(Decimal::ZERO),
            most_popular_sport: basic_row.most_popular_sport,
            top_players,
            top_teams,
            recent_tournaments: recent_tournaments.into_iter().map(TournamentStatistics::from).collect(),
            growth_metrics,
        })
    }
}
