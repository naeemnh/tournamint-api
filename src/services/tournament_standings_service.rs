use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use serde_json::json;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::{
        tournament_standings::{
            StandingsResponse, StandingEntry, StandingsUpdateRequest, ParticipantStats,
        },
    },
    repositories::{
        TournamentStandingsRepository, TournamentRepository, 
        TournamentCategoryRepository,
    },
    utils::db::with_transaction,
};

pub struct TournamentStandingsService;

impl TournamentStandingsService {
    pub async fn get_tournament_standings(pool: &DbPool, tournament_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                // Get tournament info
                let tournament = TournamentRepository::get_by_id(tx, tournament_id).await?;
                
                // Get standings
                let standings = TournamentStandingsRepository::get_by_tournament_id(tx, tournament_id).await?;
                
                let standings_entries: Vec<StandingEntry> = standings
                    .into_iter()
                    .map(|s| StandingEntry {
                        position: s.position,
                        participant_id: s.participant_id,
                        participant_name: s.participant_name,
                        participant_type: s.participant_type,
                        points: s.points,
                        matches_played: s.matches_played,
                        matches_won: s.matches_won,
                        matches_lost: s.matches_lost,
                        matches_drawn: s.matches_drawn,
                        win_percentage: if s.matches_played > 0 {
                            (s.matches_won as f64 / s.matches_played as f64) * 100.0
                        } else {
                            0.0
                        },
                        sets_won: s.sets_won,
                        sets_lost: s.sets_lost,
                        set_ratio: if s.sets_lost > 0 {
                            Some(s.sets_won as f64 / s.sets_lost as f64)
                        } else {
                            None
                        },
                        games_won: s.games_won,
                        games_lost: s.games_lost,
                        game_ratio: if s.games_lost > 0 {
                            Some(s.games_won as f64 / s.games_lost as f64)
                        } else {
                            None
                        },
                        goal_difference: s.goal_difference,
                        form: Vec::new(), // TODO: Calculate recent form
                        is_eliminated: s.is_eliminated,
                        elimination_round: s.elimination_round,
                    })
                    .collect();

                let response = StandingsResponse {
                    tournament_id,
                    category_id: None,
                    tournament_name: tournament.name,
                    category_name: None,
                    format: format!("{:?}", tournament.format),
                    standings: standings_entries,
                    last_updated: chrono::Utc::now(),
                };

                Ok(response)
            })
        })
        .await
        {
            Ok(standings_response) => {
                formatters::success_response(StatusCode::OK, standings_response, "STANDINGS_FETCHED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Tournament not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "STANDINGS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_category_standings(pool: &DbPool, category_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                // Get category info
                let category = TournamentCategoryRepository::get_by_id(tx, category_id).await?;
                let tournament = TournamentRepository::get_by_id(tx, category.tournament_id).await?;
                
                // Get standings
                let standings = TournamentStandingsRepository::get_by_category_id(tx, category_id).await?;
                
                let standings_entries: Vec<StandingEntry> = standings
                    .into_iter()
                    .map(|s| StandingEntry {
                        position: s.position,
                        participant_id: s.participant_id,
                        participant_name: s.participant_name,
                        participant_type: s.participant_type,
                        points: s.points,
                        matches_played: s.matches_played,
                        matches_won: s.matches_won,
                        matches_lost: s.matches_lost,
                        matches_drawn: s.matches_drawn,
                        win_percentage: if s.matches_played > 0 {
                            (s.matches_won as f64 / s.matches_played as f64) * 100.0
                        } else {
                            0.0
                        },
                        sets_won: s.sets_won,
                        sets_lost: s.sets_lost,
                        set_ratio: if s.sets_lost > 0 {
                            Some(s.sets_won as f64 / s.sets_lost as f64)
                        } else {
                            None
                        },
                        games_won: s.games_won,
                        games_lost: s.games_lost,
                        game_ratio: if s.games_lost > 0 {
                            Some(s.games_won as f64 / s.games_lost as f64)
                        } else {
                            None
                        },
                        goal_difference: s.goal_difference,
                        form: Vec::new(), // TODO: Calculate recent form
                        is_eliminated: s.is_eliminated,
                        elimination_round: s.elimination_round,
                    })
                    .collect();

                let response = StandingsResponse {
                    tournament_id: category.tournament_id,
                    category_id: Some(category_id),
                    tournament_name: tournament.name,
                    category_name: Some(category.name),
                    format: format!("{:?}", tournament.format),
                    standings: standings_entries,
                    last_updated: chrono::Utc::now(),
                };

                Ok(response)
            })
        })
        .await
        {
            Ok(standings_response) => {
                formatters::success_response(StatusCode::OK, standings_response, "STANDINGS_FETCHED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Category not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "STANDINGS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn update_standings(
        pool: &DbPool,
        tournament_id: Uuid,
        request: StandingsUpdateRequest,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::update_standings_internal(tx, tournament_id, request).await
            })
        })
        .await
        {
            Ok(updated_count) => {
                formatters::success_response(
                    StatusCode::OK,
                    json!({ 
                        "message": "Standings updated successfully",
                        "updated_records": updated_count 
                    }),
                    "STANDINGS_UPDATED",
                )
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "STANDINGS_UPDATE_ERROR",
                )
            }
        }
    }

    async fn update_standings_internal(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        request: StandingsUpdateRequest,
    ) -> Result<u64, sqlx::Error> {
        let mut total_updated = 0;

        if request.recalculate_all.unwrap_or(false) {
            // Recalculate all standings from scratch
            total_updated += Self::recalculate_all_standings(tx, tournament_id, request.category_id).await?;
        } else if let Some(match_ids) = request.match_ids {
            // Update standings based on specific matches
            total_updated += Self::update_standings_from_matches(tx, tournament_id, request.category_id, match_ids).await?;
        } else {
            // Default: recalculate all standings
            total_updated += Self::recalculate_all_standings(tx, tournament_id, request.category_id).await?;
        }

        // Update positions based on points and tiebreakers
        total_updated += TournamentStandingsRepository::update_positions(tx, tournament_id, request.category_id).await?;

        Ok(total_updated)
    }

    async fn recalculate_all_standings(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<u64, sqlx::Error> {
        // Get all participants for the tournament/category
        let participants = Self::get_all_participants(tx, tournament_id, category_id).await?;
        
        // Calculate stats for each participant
        let mut participant_stats = Vec::new();
        
        for (participant_id, participant_name, participant_type) in participants {
            let stats = Self::calculate_participant_stats(
                tx,
                tournament_id,
                category_id,
                participant_id,
            ).await?;
            
            participant_stats.push((
                tournament_id,
                category_id,
                participant_id,
                participant_name,
                participant_type,
                stats,
            ));
        }

        // Bulk upsert standings
        TournamentStandingsRepository::bulk_upsert(tx, participant_stats).await
    }

    async fn update_standings_from_matches(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
        match_ids: Vec<Uuid>,
    ) -> Result<u64, sqlx::Error> {
        let mut total_updated = 0;
        let mut affected_participants = std::collections::HashSet::new();

        // Find participants affected by these matches
        for match_id in match_ids {
            let participants = Self::get_match_participants(tx, match_id).await?;
            for participant_id in participants {
                affected_participants.insert(participant_id);
            }
        }

        // Update standings for affected participants
        for participant_id in affected_participants {
            let stats = Self::calculate_participant_stats(
                tx,
                tournament_id,
                category_id,
                participant_id,
            ).await?;

            // Get participant name and type
            let (participant_name, participant_type) = Self::get_participant_details(tx, participant_id).await?;

            let standings_data = vec![(
                tournament_id,
                category_id,
                participant_id,
                participant_name,
                participant_type,
                stats,
            )];

            total_updated += TournamentStandingsRepository::bulk_upsert(tx, standings_data).await?;
        }

        Ok(total_updated)
    }

    async fn get_all_participants(
        _tx: &mut sqlx::PgConnection,
        _tournament_id: Uuid,
        _category_id: Option<Uuid>,
    ) -> Result<Vec<(Uuid, String, String)>, sqlx::Error> {
        // TODO: Implement with proper sea-query when tournament_registrations table exists
        // For now, return dummy data for compilation
        Ok(vec![
            (Uuid::new_v4(), "Team A".to_string(), "team".to_string()),
            (Uuid::new_v4(), "Team B".to_string(), "team".to_string()),
        ])
    }

    async fn calculate_participant_stats(
        _tx: &mut sqlx::PgConnection,
        _tournament_id: Uuid,
        _category_id: Option<Uuid>,
        participant_id: Uuid,
    ) -> Result<ParticipantStats, sqlx::Error> {
        // TODO: Implement with proper sea-query when matches table exists
        // For now, return default stats for compilation
        Ok(ParticipantStats {
            participant_id,
            matches_played: 0,
            matches_won: 0,
            matches_lost: 0,
            matches_drawn: 0,
            sets_won: 0,
            sets_lost: 0,
            games_won: 0,
            games_lost: 0,
            points_scored: 0,
            points_conceded: 0,
        })
    }

    async fn get_match_participants(
        _tx: &mut sqlx::PgConnection,
        _match_id: Uuid,
    ) -> Result<Vec<Uuid>, sqlx::Error> {
        // TODO: Implement with proper sea-query when matches table exists
        // For now, return empty vector for compilation
        Ok(vec![])
    }

    async fn get_participant_details(
        _tx: &mut sqlx::PgConnection,
        _participant_id: Uuid,
    ) -> Result<(String, String), sqlx::Error> {
        // TODO: Implement with proper sea-query when teams/players tables exist
        // For now, return default data for compilation
        Ok(("Unknown Participant".to_string(), "team".to_string()))
    }
}