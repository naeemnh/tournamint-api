use sqlx::{PgConnection, Row};
use uuid::Uuid;

use crate::models::statistics::{
    PlayerStatistics, TeamStatistics, TournamentStatistics, LeaderboardEntry,
    GameRecord, AnalyticsDashboard, GrowthMetrics, LeaderboardRequest,
};

pub struct StatisticsRepository;

impl StatisticsRepository {
    // Player Statistics
    pub async fn get_player_statistics(
        tx: &mut PgConnection,
        player_id: Uuid,
    ) -> Result<Option<PlayerStatistics>, sqlx::Error> {
        let sql = r#"
            WITH player_info AS (
                SELECT p.id as player_id, p.name as player_name
                FROM players p
                WHERE p.id = $1
            ),
            tournament_stats AS (
                SELECT 
                    COUNT(*) as total_tournaments,
                    COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) as tournaments_won,
                    COUNT(CASE WHEN tr.final_position = 2 THEN 1 END) as tournaments_runner_up,
                    COALESCE(AVG(tr.final_position), 0) as avg_placement,
                    COALESCE(MIN(tr.final_position), 999) as best_placement,
                    COALESCE(SUM(tr.prize_amount), 0) as total_earnings
                FROM tournament_registrations tr
                WHERE tr.player_id = $1 AND tr.status = 'confirmed'
            ),
            match_stats AS (
                SELECT 
                    COUNT(*) as total_matches,
                    COUNT(CASE WHEN mr.winner_id = $1 THEN 1 END) as matches_won,
                    COUNT(CASE WHEN mr.winner_id != $1 AND mr.winner_id IS NOT NULL THEN 1 END) as matches_lost
                FROM matches m
                JOIN match_results mr ON m.id = mr.match_id
                WHERE (m.player1_id = $1 OR m.player2_id = $1) 
                  AND mr.status = 'completed'
            )
            SELECT 
                pi.player_id,
                pi.player_name,
                COALESCE(ts.total_tournaments, 0) as total_tournaments,
                COALESCE(ts.tournaments_won, 0) as tournaments_won,
                COALESCE(ts.tournaments_runner_up, 0) as tournaments_runner_up,
                COALESCE(ms.total_matches, 0) as total_matches,
                COALESCE(ms.matches_won, 0) as matches_won,
                COALESCE(ms.matches_lost, 0) as matches_lost,
                CASE 
                    WHEN COALESCE(ms.total_matches, 0) = 0 THEN 0
                    ELSE ROUND(CAST(ms.matches_won AS DECIMAL) / ms.total_matches * 100, 2)
                END as win_rate,
                COALESCE(ts.total_earnings, 0) as total_earnings,
                ts.avg_placement as average_placement,
                CASE WHEN ts.best_placement = 999 THEN 0 ELSE ts.best_placement END as best_placement,
                NULL::INTEGER as current_ranking,
                COALESCE(ts.tournaments_won * 100 + ms.matches_won * 10, 0) as ranking_points,
                COALESCE(
                    (SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = $1),
                    NOW()
                ) as last_active
            FROM player_info pi
            LEFT JOIN tournament_stats ts ON true
            LEFT JOIN match_stats ms ON true
        "#;

        sqlx::query_as::<_, PlayerStatistics>(sql)
            .bind(player_id)
            .fetch_optional(&mut *tx)
            .await
    }

    // Team Statistics
    pub async fn get_team_statistics(
        tx: &mut PgConnection,
        team_id: Uuid,
    ) -> Result<Option<TeamStatistics>, sqlx::Error> {
        let sql = r#"
            WITH team_info AS (
                SELECT t.id as team_id, t.name as team_name
                FROM teams t
                WHERE t.id = $1
            ),
            tournament_stats AS (
                SELECT 
                    COUNT(*) as total_tournaments,
                    COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) as tournaments_won,
                    COUNT(CASE WHEN tr.final_position = 2 THEN 1 END) as tournaments_runner_up,
                    COALESCE(AVG(tr.final_position), 0) as avg_placement,
                    COALESCE(MIN(tr.final_position), 999) as best_placement,
                    COALESCE(SUM(tr.prize_amount), 0) as total_earnings
                FROM tournament_registrations tr
                WHERE tr.team_id = $1 AND tr.status = 'confirmed'
            ),
            match_stats AS (
                SELECT 
                    COUNT(*) as total_matches,
                    COUNT(CASE WHEN mr.winner_id = $1 THEN 1 END) as matches_won,
                    COUNT(CASE WHEN mr.winner_id != $1 AND mr.winner_id IS NOT NULL THEN 1 END) as matches_lost
                FROM matches m
                JOIN match_results mr ON m.id = mr.match_id
                WHERE (m.team1_id = $1 OR m.team2_id = $1) 
                  AND mr.status = 'completed'
            ),
            member_count AS (
                SELECT COUNT(*) as members_count
                FROM team_members tm
                WHERE tm.team_id = $1 AND tm.status = 'active'
            )
            SELECT 
                ti.team_id,
                ti.team_name,
                COALESCE(ts.total_tournaments, 0) as total_tournaments,
                COALESCE(ts.tournaments_won, 0) as tournaments_won,
                COALESCE(ts.tournaments_runner_up, 0) as tournaments_runner_up,
                COALESCE(ms.total_matches, 0) as total_matches,
                COALESCE(ms.matches_won, 0) as matches_won,
                COALESCE(ms.matches_lost, 0) as matches_lost,
                CASE 
                    WHEN COALESCE(ms.total_matches, 0) = 0 THEN 0
                    ELSE ROUND(CAST(ms.matches_won AS DECIMAL) / ms.total_matches * 100, 2)
                END as win_rate,
                COALESCE(ts.total_earnings, 0) as total_earnings,
                ts.avg_placement as average_placement,
                CASE WHEN ts.best_placement = 999 THEN 0 ELSE ts.best_placement END as best_placement,
                NULL::INTEGER as current_ranking,
                COALESCE(ts.tournaments_won * 100 + ms.matches_won * 10, 0) as ranking_points,
                mc.members_count,
                COALESCE(
                    (SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.team_id = $1),
                    NOW()
                ) as last_active
            FROM team_info ti
            LEFT JOIN tournament_stats ts ON true
            LEFT JOIN match_stats ms ON true
            LEFT JOIN member_count mc ON true
        "#;

        sqlx::query_as::<_, TeamStatistics>(sql)
            .bind(team_id)
            .fetch_optional(&mut *tx)
            .await
    }

    // Tournament Statistics
    pub async fn get_tournament_statistics(
        tx: &mut PgConnection,
        tournament_id: Uuid,
    ) -> Result<Option<TournamentStatistics>, sqlx::Error> {
        let sql = r#"
            WITH tournament_info AS (
                SELECT t.id, t.name, t.start_date, t.end_date, t.prize_pool
                FROM tournaments t
                WHERE t.id = $1
            ),
            registration_stats AS (
                SELECT 
                    COUNT(*) as total_registrations,
                    COUNT(CASE WHEN player_id IS NOT NULL THEN 1 END) as total_participants,
                    COUNT(CASE WHEN team_id IS NOT NULL THEN 1 END) as total_teams
                FROM tournament_registrations tr
                WHERE tr.tournament_id = $1 AND tr.status = 'confirmed'
            ),
            match_stats AS (
                SELECT 
                    COUNT(*) as total_matches,
                    COUNT(CASE WHEN mr.status = 'completed' THEN 1 END) as completed_matches,
                    COUNT(CASE WHEN mr.status IN ('pending', 'in_progress') THEN 1 END) as pending_matches,
                    AVG(EXTRACT(EPOCH FROM (mr.updated_at - mr.created_at)) / 60)::INTEGER as avg_duration
                FROM matches m
                LEFT JOIN match_results mr ON m.id = mr.match_id
                WHERE m.tournament_id = $1
            ),
            winner_stats AS (
                SELECT 
                    (SELECT p.name FROM players p 
                     JOIN match_results mr ON p.id = mr.winner_id 
                     WHERE mr.match_id IN (SELECT id FROM matches WHERE tournament_id = $1)
                     GROUP BY p.id, p.name 
                     ORDER BY COUNT(*) DESC 
                     LIMIT 1) as most_wins_player,
                    (SELECT t.name FROM teams t 
                     JOIN match_results mr ON t.id = mr.winner_id 
                     WHERE mr.match_id IN (SELECT id FROM matches WHERE tournament_id = $1)
                     GROUP BY t.id, t.name 
                     ORDER BY COUNT(*) DESC 
                     LIMIT 1) as most_wins_team
            )
            SELECT 
                ti.id as tournament_id,
                ti.name as tournament_name,
                COALESCE(rs.total_participants, 0) as total_participants,
                COALESCE(rs.total_teams, 0) as total_teams,
                COALESCE(ms.total_matches, 0) as total_matches,
                COALESCE(ms.completed_matches, 0) as completed_matches,
                COALESCE(ms.pending_matches, 0) as pending_matches,
                COALESCE(ti.prize_pool, 0) as total_prize_pool,
                COALESCE(rs.total_registrations, 0) as total_registrations,
                CASE 
                    WHEN COALESCE(ms.total_matches, 0) = 0 THEN 0
                    ELSE ROUND(CAST(ms.completed_matches AS DECIMAL) / ms.total_matches * 100, 2)
                END as completion_rate,
                ms.avg_duration as average_match_duration,
                ws.most_wins_player,
                ws.most_wins_team,
                ti.start_date,
                ti.end_date
            FROM tournament_info ti
            LEFT JOIN registration_stats rs ON true
            LEFT JOIN match_stats ms ON true
            LEFT JOIN winner_stats ws ON true
        "#;

        sqlx::query_as::<_, TournamentStatistics>(sql)
            .bind(tournament_id)
            .fetch_optional(&mut *tx)
            .await
    }

    // Leaderboard
    pub async fn get_leaderboard(
        tx: &mut PgConnection,
        request: &LeaderboardRequest,
    ) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
        let limit = request.limit.unwrap_or(50).min(100);
        let offset = request.offset.unwrap_or(0);

        let sql = match request.entity_type.as_str() {
            "player" => {
                match request.category.as_str() {
                    "points" => r#"
                        WITH player_stats AS (
                            SELECT 
                                p.id,
                                p.name,
                                COALESCE(
                                    (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed') * 100 +
                                    (SELECT COUNT(CASE WHEN mr.winner_id = p.id THEN 1 END) FROM match_results mr 
                                     JOIN matches m ON mr.match_id = m.id 
                                     WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') * 10,
                                    0
                                ) as ranking_points,
                                (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed') as tournaments_won,
                                CASE 
                                    WHEN (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') = 0 THEN 0
                                    ELSE ROUND(
                                        CAST((SELECT COUNT(CASE WHEN mr.winner_id = p.id THEN 1 END) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') AS DECIMAL) / 
                                        (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') * 100, 2
                                    )
                                END as win_rate,
                                COALESCE((SELECT SUM(tr.prize_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed'), 0) as total_earnings,
                                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) as last_active
                            FROM players p
                        )
                        SELECT 
                            ROW_NUMBER() OVER (ORDER BY ranking_points DESC, tournaments_won DESC) as rank,
                            id,
                            name,
                            ranking_points as points,
                            tournaments_won,
                            win_rate,
                            total_earnings,
                            last_active
                        FROM player_stats
                        WHERE ranking_points > 0
                        ORDER BY ranking_points DESC, tournaments_won DESC
                        LIMIT $1 OFFSET $2
                    "#,
                    "wins" => r#"
                        WITH player_stats AS (
                            SELECT 
                                p.id,
                                p.name,
                                (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed') as tournaments_won,
                                CASE 
                                    WHEN (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') = 0 THEN 0
                                    ELSE ROUND(
                                        CAST((SELECT COUNT(CASE WHEN mr.winner_id = p.id THEN 1 END) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') AS DECIMAL) / 
                                        (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') * 100, 2
                                    )
                                END as win_rate,
                                COALESCE((SELECT SUM(tr.prize_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed'), 0) as total_earnings,
                                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) as last_active
                            FROM players p
                        )
                        SELECT 
                            ROW_NUMBER() OVER (ORDER BY tournaments_won DESC, win_rate DESC) as rank,
                            id,
                            name,
                            tournaments_won as points,
                            tournaments_won,
                            win_rate,
                            total_earnings,
                            last_active
                        FROM player_stats
                        WHERE tournaments_won > 0
                        ORDER BY tournaments_won DESC, win_rate DESC
                        LIMIT $1 OFFSET $2
                    "#,
                    "earnings" => r#"
                        WITH player_stats AS (
                            SELECT 
                                p.id,
                                p.name,
                                COALESCE((SELECT SUM(tr.prize_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed'), 0) as total_earnings,
                                (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed') as tournaments_won,
                                CASE 
                                    WHEN (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') = 0 THEN 0
                                    ELSE ROUND(
                                        CAST((SELECT COUNT(CASE WHEN mr.winner_id = p.id THEN 1 END) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') AS DECIMAL) / 
                                        (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') * 100, 2
                                    )
                                END as win_rate,
                                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) as last_active
                            FROM players p
                        )
                        SELECT 
                            ROW_NUMBER() OVER (ORDER BY total_earnings DESC, tournaments_won DESC) as rank,
                            id,
                            name,
                            total_earnings as points,
                            tournaments_won,
                            win_rate,
                            total_earnings,
                            last_active
                        FROM player_stats
                        WHERE total_earnings > 0
                        ORDER BY total_earnings DESC, tournaments_won DESC
                        LIMIT $1 OFFSET $2
                    "#,
                    _ => r#"
                        WITH player_stats AS (
                            SELECT 
                                p.id,
                                p.name,
                                CASE 
                                    WHEN (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') = 0 THEN 0
                                    ELSE ROUND(
                                        CAST((SELECT COUNT(CASE WHEN mr.winner_id = p.id THEN 1 END) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') AS DECIMAL) / 
                                        (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') * 100, 2
                                    )
                                END as win_rate,
                                (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed') as tournaments_won,
                                COALESCE((SELECT SUM(tr.prize_amount) FROM tournament_registrations tr WHERE tr.player_id = p.id AND tr.status = 'confirmed'), 0) as total_earnings,
                                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.player_id = p.id), p.created_at) as last_active
                            FROM players p
                        )
                        SELECT 
                            ROW_NUMBER() OVER (ORDER BY win_rate DESC, tournaments_won DESC) as rank,
                            id,
                            name,
                            win_rate as points,
                            tournaments_won,
                            win_rate,
                            total_earnings,
                            last_active
                        FROM player_stats
                        WHERE (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.player1_id = p.id OR m.player2_id = p.id) AND mr.status = 'completed') > 0
                        ORDER BY win_rate DESC, tournaments_won DESC
                        LIMIT $1 OFFSET $2
                    "#,
                }
            }
            _ => {
                // Team leaderboard queries (similar structure)
                match request.category.as_str() {
                    "points" => r#"
                        WITH team_stats AS (
                            SELECT 
                                t.id,
                                t.name,
                                COALESCE(
                                    (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.team_id = t.id AND tr.status = 'confirmed') * 100 +
                                    (SELECT COUNT(CASE WHEN mr.winner_id = t.id THEN 1 END) FROM match_results mr 
                                     JOIN matches m ON mr.match_id = m.id 
                                     WHERE (m.team1_id = t.id OR m.team2_id = t.id) AND mr.status = 'completed') * 10,
                                    0
                                ) as ranking_points,
                                (SELECT COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) FROM tournament_registrations tr WHERE tr.team_id = t.id AND tr.status = 'confirmed') as tournaments_won,
                                CASE 
                                    WHEN (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.team1_id = t.id OR m.team2_id = t.id) AND mr.status = 'completed') = 0 THEN 0
                                    ELSE ROUND(
                                        CAST((SELECT COUNT(CASE WHEN mr.winner_id = t.id THEN 1 END) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.team1_id = t.id OR m.team2_id = t.id) AND mr.status = 'completed') AS DECIMAL) / 
                                        (SELECT COUNT(*) FROM match_results mr JOIN matches m ON mr.match_id = m.id WHERE (m.team1_id = t.id OR m.team2_id = t.id) AND mr.status = 'completed') * 100, 2
                                    )
                                END as win_rate,
                                COALESCE((SELECT SUM(tr.prize_amount) FROM tournament_registrations tr WHERE tr.team_id = t.id AND tr.status = 'confirmed'), 0) as total_earnings,
                                COALESCE((SELECT MAX(tr.updated_at) FROM tournament_registrations tr WHERE tr.team_id = t.id), t.created_at) as last_active
                            FROM teams t
                        )
                        SELECT 
                            ROW_NUMBER() OVER (ORDER BY ranking_points DESC, tournaments_won DESC) as rank,
                            id,
                            name,
                            ranking_points as points,
                            tournaments_won,
                            win_rate,
                            total_earnings,
                            last_active
                        FROM team_stats
                        WHERE ranking_points > 0
                        ORDER BY ranking_points DESC, tournaments_won DESC
                        LIMIT $1 OFFSET $2
                    "#,
                    _ => r#"
                        SELECT 
                            1::BIGINT as rank,
                            gen_random_uuid() as id,
                            'Sample Team' as name,
                            0::DECIMAL as points,
                            0::BIGINT as tournaments_won,
                            0::DECIMAL as win_rate,
                            0::DECIMAL as total_earnings,
                            NOW() as last_active
                        WHERE FALSE
                        LIMIT $1 OFFSET $2
                    "#,
                }
            }
        };

        sqlx::query_as::<_, LeaderboardEntry>(sql)
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut *tx)
            .await
    }

    // Game Records
    pub async fn get_game_records(
        tx: &mut PgConnection,
        limit: Option<i64>,
    ) -> Result<Vec<GameRecord>, sqlx::Error> {
        let limit = limit.unwrap_or(20).min(100);

        let sql = r#"
            WITH records AS (
                -- Most tournament wins (players)
                SELECT 
                    gen_random_uuid() as id,
                    'most_tournament_wins' as category,
                    'player' as record_type,
                    p.id as holder_id,
                    p.name as holder_name,
                    COUNT(CASE WHEN tr.final_position = 1 THEN 1 END)::DECIMAL as value,
                    'Most tournament victories' as description,
                    MAX(tr.updated_at) as achieved_date,
                    NULL::UUID as tournament_id,
                    NULL as tournament_name
                FROM players p
                JOIN tournament_registrations tr ON p.id = tr.player_id
                WHERE tr.status = 'confirmed' AND tr.final_position = 1
                GROUP BY p.id, p.name
                HAVING COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) > 0
                ORDER BY value DESC
                LIMIT 1
                
                UNION ALL
                
                -- Highest single tournament earnings (players)
                SELECT 
                    gen_random_uuid() as id,
                    'highest_single_earnings' as category,
                    'player' as record_type,
                    p.id as holder_id,
                    p.name as holder_name,
                    MAX(tr.prize_amount)::DECIMAL as value,
                    'Highest single tournament earnings' as description,
                    (SELECT tr2.updated_at FROM tournament_registrations tr2 WHERE tr2.player_id = p.id AND tr2.prize_amount = MAX(tr.prize_amount) LIMIT 1) as achieved_date,
                    (SELECT tr3.tournament_id FROM tournament_registrations tr3 WHERE tr3.player_id = p.id AND tr3.prize_amount = MAX(tr.prize_amount) LIMIT 1) as tournament_id,
                    (SELECT t.name FROM tournaments t JOIN tournament_registrations tr4 ON t.id = tr4.tournament_id WHERE tr4.player_id = p.id AND tr4.prize_amount = MAX(tr.prize_amount) LIMIT 1) as tournament_name
                FROM players p
                JOIN tournament_registrations tr ON p.id = tr.player_id
                WHERE tr.status = 'confirmed' AND tr.prize_amount > 0
                GROUP BY p.id, p.name
                ORDER BY value DESC
                LIMIT 1
                
                UNION ALL
                
                -- Most tournament wins (teams)
                SELECT 
                    gen_random_uuid() as id,
                    'most_tournament_wins' as category,
                    'team' as record_type,
                    t.id as holder_id,
                    t.name as holder_name,
                    COUNT(CASE WHEN tr.final_position = 1 THEN 1 END)::DECIMAL as value,
                    'Most tournament victories (team)' as description,
                    MAX(tr.updated_at) as achieved_date,
                    NULL::UUID as tournament_id,
                    NULL as tournament_name
                FROM teams t
                JOIN tournament_registrations tr ON t.id = tr.team_id
                WHERE tr.status = 'confirmed' AND tr.final_position = 1
                GROUP BY t.id, t.name
                HAVING COUNT(CASE WHEN tr.final_position = 1 THEN 1 END) > 0
                ORDER BY value DESC
                LIMIT 1
            )
            SELECT * FROM records
            ORDER BY value DESC, achieved_date DESC
            LIMIT $1
        "#;

        sqlx::query_as::<_, GameRecord>(sql)
            .bind(limit)
            .fetch_all(&mut *tx)
            .await
    }

    // Growth Metrics
    pub async fn get_growth_metrics(
        tx: &mut PgConnection,
    ) -> Result<GrowthMetrics, sqlx::Error> {
        let sql = r#"
            WITH current_month AS (
                SELECT 
                    COUNT(CASE WHEN DATE_TRUNC('month', p.created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN 1 END) as new_players_this_month,
                    COUNT(CASE WHEN DATE_TRUNC('month', t.created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN 1 END) as new_teams_this_month
                FROM players p
                FULL OUTER JOIN teams t ON FALSE
            ),
            tournament_metrics AS (
                SELECT 
                    COUNT(CASE WHEN DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN 1 END) as tournaments_this_month,
                    0 as matches_this_month, -- Placeholder
                    COALESCE(SUM(CASE WHEN DATE_TRUNC('month', created_at) = DATE_TRUNC('month', CURRENT_DATE) THEN prize_pool ELSE 0 END), 0) as revenue_this_month
                FROM tournaments
            ),
            previous_month AS (
                SELECT 
                    COUNT(CASE WHEN DATE_TRUNC('month', p.created_at) = DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month') THEN 1 END) as prev_players,
                    COUNT(CASE WHEN DATE_TRUNC('month', t.created_at) = DATE_TRUNC('month', CURRENT_DATE - INTERVAL '1 month') THEN 1 END) as prev_tournaments
                FROM players p
                FULL OUTER JOIN tournaments t ON FALSE
            )
            SELECT 
                cm.new_players_this_month,
                cm.new_teams_this_month,
                tm.tournaments_this_month,
                tm.matches_this_month,
                tm.revenue_this_month,
                CASE 
                    WHEN pm.prev_players = 0 THEN 0 
                    ELSE ROUND(((cm.new_players_this_month::DECIMAL - pm.prev_players) / pm.prev_players * 100), 2)
                END as player_growth_rate,
                CASE 
                    WHEN pm.prev_tournaments = 0 THEN 0 
                    ELSE ROUND(((tm.tournaments_this_month::DECIMAL - pm.prev_tournaments) / pm.prev_tournaments * 100), 2)
                END as tournament_growth_rate
            FROM current_month cm
            CROSS JOIN tournament_metrics tm
            CROSS JOIN previous_month pm
        "#;

        sqlx::query_as::<_, GrowthMetrics>(sql)
            .fetch_one(&mut *tx)
            .await
    }

    // Analytics Dashboard Data
    pub async fn get_analytics_dashboard(
        tx: &mut PgConnection,
    ) -> Result<AnalyticsDashboard, sqlx::Error> {
        // Get basic counts
        let basic_stats_sql = r#"
            SELECT 
                (SELECT COUNT(*) FROM players) as total_players,
                (SELECT COUNT(*) FROM teams) as total_teams,
                (SELECT COUNT(*) FROM tournaments) as total_tournaments,
                (SELECT COUNT(*) FROM tournaments WHERE status = 'active') as active_tournaments,
                (SELECT COUNT(*) FROM matches) as total_matches,
                COALESCE((SELECT SUM(prize_pool) FROM tournaments WHERE status = 'completed'), 0) as total_earnings_distributed,
                CASE 
                    WHEN (SELECT COUNT(*) FROM tournaments) = 0 THEN 0
                    ELSE (SELECT AVG(
                        (SELECT COUNT(*) FROM tournament_registrations tr2 WHERE tr2.tournament_id = t.id AND tr2.status = 'confirmed')
                    ) FROM tournaments t)
                END as average_tournament_size,
                (SELECT sport_type FROM tournaments GROUP BY sport_type ORDER BY COUNT(*) DESC LIMIT 1) as most_popular_sport
        "#;

        let basic_row = sqlx::query(basic_stats_sql)
            .fetch_one(&mut *tx)
            .await?;

        // Get growth metrics
        let growth_metrics = Self::get_growth_metrics(tx).await?;

        // Get top players
        let player_leaderboard_request = LeaderboardRequest {
            category: "points".to_string(),
            entity_type: "player".to_string(),
            sport_type: None,
            limit: Some(5),
            offset: Some(0),
        };
        let top_players = Self::get_leaderboard(tx, &player_leaderboard_request).await?;

        // Get top teams
        let team_leaderboard_request = LeaderboardRequest {
            category: "points".to_string(),
            entity_type: "team".to_string(),
            sport_type: None,
            limit: Some(5),
            offset: Some(0),
        };
        let top_teams = Self::get_leaderboard(tx, &team_leaderboard_request).await?;

        // Get recent tournaments (simplified)
        let recent_tournaments_sql = r#"
            SELECT 
                t.id as tournament_id,
                t.name as tournament_name,
                (SELECT COUNT(*) FROM tournament_registrations tr WHERE tr.tournament_id = t.id AND tr.status = 'confirmed' AND tr.player_id IS NOT NULL) as total_participants,
                (SELECT COUNT(*) FROM tournament_registrations tr WHERE tr.tournament_id = t.id AND tr.status = 'confirmed' AND tr.team_id IS NOT NULL) as total_teams,
                (SELECT COUNT(*) FROM matches m WHERE m.tournament_id = t.id) as total_matches,
                (SELECT COUNT(*) FROM matches m JOIN match_results mr ON m.id = mr.match_id WHERE m.tournament_id = t.id AND mr.status = 'completed') as completed_matches,
                (SELECT COUNT(*) FROM matches m LEFT JOIN match_results mr ON m.id = mr.match_id WHERE m.tournament_id = t.id AND (mr.status IS NULL OR mr.status IN ('pending', 'in_progress'))) as pending_matches,
                COALESCE(t.prize_pool, 0) as total_prize_pool,
                (SELECT COUNT(*) FROM tournament_registrations tr WHERE tr.tournament_id = t.id) as total_registrations,
                CASE 
                    WHEN (SELECT COUNT(*) FROM matches m WHERE m.tournament_id = t.id) = 0 THEN 0
                    ELSE ROUND(
                        CAST((SELECT COUNT(*) FROM matches m JOIN match_results mr ON m.id = mr.match_id WHERE m.tournament_id = t.id AND mr.status = 'completed') AS DECIMAL) / 
                        (SELECT COUNT(*) FROM matches m WHERE m.tournament_id = t.id) * 100, 2
                    )
                END as completion_rate,
                NULL::INTEGER as average_match_duration,
                NULL as most_wins_player,
                NULL as most_wins_team,
                t.start_date,
                t.end_date
            FROM tournaments t
            ORDER BY t.created_at DESC
            LIMIT 5
        "#;

        let recent_tournaments = sqlx::query_as::<_, TournamentStatistics>(recent_tournaments_sql)
            .fetch_all(&mut *tx)
            .await?;

        Ok(AnalyticsDashboard {
            total_players: basic_row.get("total_players"),
            total_teams: basic_row.get("total_teams"),
            total_tournaments: basic_row.get("total_tournaments"),
            active_tournaments: basic_row.get("active_tournaments"),
            total_matches: basic_row.get("total_matches"),
            total_earnings_distributed: basic_row.get("total_earnings_distributed"),
            average_tournament_size: basic_row.get("average_tournament_size"),
            most_popular_sport: basic_row.get("most_popular_sport"),
            top_players,
            top_teams,
            recent_tournaments,
            growth_metrics,
        })
    }
}