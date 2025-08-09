use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use rust_decimal::Decimal;
use serde_json::{json, Value as JsonValue};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::{
        tournament_bracket::{
            BracketType, BracketStatus, GenerateBracketRequest, NewTournamentBracket,
            EditableTournamentBracket, BracketResponse, BracketMatch, BracketParticipant,
            BracketNode,
        },
        match_model::{MatchType, MatchStatus, NewMatch},
    },
    repositories::{
        TournamentBracketRepository,
        MatchRepository,
    },
    utils::db::with_transaction,
};

pub struct TournamentBracketService;

impl TournamentBracketService {
    pub async fn get_tournament_bracket(pool: &DbPool, tournament_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentBracketRepository::get_by_tournament_id(tx, tournament_id).await
            })
        })
        .await
        {
            Ok(brackets) => {
                formatters::success_response(StatusCode::OK, brackets, "BRACKETS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "BRACKETS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_category_bracket(pool: &DbPool, category_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentBracketRepository::get_by_category_id(tx, category_id).await
            })
        })
        .await
        {
            Ok(bracket_opt) => match bracket_opt {
                Some(bracket) => {
                    // Get detailed bracket information with matches
                    match Self::get_bracket_details(pool, bracket.id).await {
                        Ok(response) => {
                            formatters::success_response(StatusCode::OK, response, "BRACKET_FETCHED")
                        }
                        Err(e) => {
                            formatters::error_response(
                                StatusCode::INTERNAL_SERVER_ERROR,
                                &e.to_string(),
                                "BRACKET_DETAILS_ERROR",
                            )
                        }
                    }
                }
                None => {
                    formatters::error_response(
                        StatusCode::NOT_FOUND,
                        "Bracket not found for this category",
                        "BRACKET_NOT_FOUND",
                    )
                }
            },
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "BRACKET_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn generate_bracket(
        pool: &DbPool,
        tournament_id: Uuid,
        request: GenerateBracketRequest,
    ) -> HttpResponse {
        // Check if bracket already exists
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentBracketRepository::exists_for_tournament(
                    tx,
                    tournament_id,
                    request.category_id,
                )
                .await
            })
        })
        .await
        {
            Ok(exists) => {
                if exists {
                    return formatters::error_response(
                        StatusCode::CONFLICT,
                        "Bracket already exists for this tournament/category",
                        "BRACKET_EXISTS",
                    );
                }
            }
            Err(e) => {
                return formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &e.to_string(),
                    "BRACKET_CHECK_ERROR",
                );
            }
        }

        match with_transaction(pool, |tx| {
            Box::pin(async move {
                Self::generate_bracket_internal(tx, tournament_id, request).await
            })
        })
        .await
        {
            Ok(bracket) => {
                formatters::success_response(StatusCode::CREATED, bracket, "BRACKET_GENERATED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    &error_message,
                    "BRACKET_GENERATION_ERROR",
                )
            }
        }
    }

    async fn generate_bracket_internal(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        request: GenerateBracketRequest,
    ) -> Result<BracketResponse, sqlx::Error> {
        // Get participants for the tournament/category
        let participants = Self::get_participants(tx, tournament_id, request.category_id).await?;

        if participants.is_empty() {
            return Err(sqlx::Error::RowNotFound);
        }

        // Generate bracket structure based on type
        let (bracket_data, total_rounds) = match request.bracket_type {
            BracketType::SingleElimination => {
                Self::generate_single_elimination(&participants, request.seed_order)?
            }
            BracketType::DoubleElimination => {
                Self::generate_double_elimination(&participants, request.seed_order)?
            }
            BracketType::RoundRobin => {
                Self::generate_round_robin(&participants)?
            }
            _ => return Err(sqlx::Error::RowNotFound),
        };

        // Create bracket record
        let new_bracket = NewTournamentBracket {
            tournament_id,
            category_id: request.category_id,
            bracket_type: request.bracket_type,
            total_rounds,
            bracket_data: Some(bracket_data.clone()),
            settings: request.settings,
        };

        let bracket = TournamentBracketRepository::create(tx, new_bracket).await?;

        // Create matches for the bracket
        let matches = Self::create_matches_from_bracket(
            tx,
            tournament_id,
            request.category_id,
            &bracket_data,
        )
        .await?;

        // Update bracket status to generated
        let updated_bracket = TournamentBracketRepository::update_status(
            tx,
            bracket.id,
            BracketStatus::Generated,
        )
        .await?;

        Ok(BracketResponse {
            bracket: updated_bracket,
            matches,
            participants: participants
                .into_iter()
                .enumerate()
                .map(|(index, p)| BracketParticipant {
                    id: p.0,
                    name: p.1,
                    seed: Some(index as i32 + 1),
                    eliminated: false,
                    current_round: Some(1),
                })
                .collect(),
        })
    }

    async fn get_participants(
        _tx: &mut sqlx::PgConnection,
        _tournament_id: Uuid,
        _category_id: Option<Uuid>,
    ) -> Result<Vec<(Uuid, String)>, sqlx::Error> {
        // TODO: Implement with proper sea-query when tournament_registrations table exists
        // For now, return dummy data for compilation
        Ok(vec![
            (Uuid::new_v4(), "Team A".to_string()),
            (Uuid::new_v4(), "Team B".to_string()),
            (Uuid::new_v4(), "Team C".to_string()),
            (Uuid::new_v4(), "Team D".to_string()),
        ])
    }

    fn generate_single_elimination(
        participants: &[(Uuid, String)],
        seed_order: Option<Vec<Uuid>>,
    ) -> Result<(JsonValue, i32), sqlx::Error> {
        let mut ordered_participants = participants.to_vec();

        // Apply custom seeding if provided
        if let Some(seeds) = seed_order {
            let mut seeded = Vec::new();
            for seed_id in seeds {
                if let Some(pos) = participants.iter().position(|(id, _)| *id == seed_id) {
                    seeded.push(participants[pos].clone());
                }
            }
            // Add any remaining participants
            for participant in participants {
                if !seeded.iter().any(|(id, _)| *id == participant.0) {
                    seeded.push(participant.clone());
                }
            }
            ordered_participants = seeded;
        }

        let participant_count = ordered_participants.len();
        let total_rounds = (participant_count as f64).log2().ceil() as i32;

        // Create bracket nodes
        let mut nodes = Vec::new();
        let mut match_number = 1;

        // First round
        let mut round = 1;
        let mut current_participants = ordered_participants.clone();

        while current_participants.len() > 1 {
            let mut next_round_participants = Vec::new();

            for i in (0..current_participants.len()).step_by(2) {
                let participant1 = current_participants.get(i).cloned();
                let participant2 = current_participants.get(i + 1).cloned();

                let node = BracketNode {
                    id: format!("round_{}_match_{}", round, match_number),
                    round,
                    match_id: None,
                    participant1_id: participant1.as_ref().map(|(id, _)| *id),
                    participant1_name: participant1.as_ref().map(|(_, name)| name.clone()),
                    participant2_id: participant2.as_ref().map(|(id, _)| *id),
                    participant2_name: participant2.as_ref().map(|(_, name)| name.clone()),
                    winner_id: None,
                    next_match_id: if round < total_rounds {
                        Some(format!("round_{}_match_{}", round + 1, (match_number + 1) / 2))
                    } else {
                        None
                    },
                    position: match_number,
                };

                nodes.push(node);
                match_number += 1;

                // For now, add a placeholder for the winner
                if participant1.is_some() || participant2.is_some() {
                    next_round_participants.push((Uuid::new_v4(), "TBD".to_string()));
                }
            }

            current_participants = next_round_participants;
            round += 1;
            if round > total_rounds {
                break;
            }
        }

        Ok((json!({ "nodes": nodes }), total_rounds))
    }

    fn generate_double_elimination(
        participants: &[(Uuid, String)],
        _seed_order: Option<Vec<Uuid>>,
    ) -> Result<(JsonValue, i32), sqlx::Error> {
        // Simplified double elimination - could be expanded
        let (single_bracket, rounds) = Self::generate_single_elimination(participants, None)?;
        
        // Double elimination needs winners and losers brackets
        // For now, return single elimination structure
        // TODO: Implement full double elimination logic
        Ok((json!({
            "winners_bracket": single_bracket,
            "losers_bracket": {},
            "grand_final": {}
        }), rounds + 1))
    }

    fn generate_round_robin(
        participants: &[(Uuid, String)],
    ) -> Result<(JsonValue, i32), sqlx::Error> {
        let participant_count = participants.len();
        if participant_count < 2 {
            return Err(sqlx::Error::RowNotFound);
        }

        let mut matches = Vec::new();
        let mut match_number = 1;
        let total_rounds = if participant_count % 2 == 0 {
            participant_count - 1
        } else {
            participant_count
        };

        // Generate all pairings
        for i in 0..participant_count {
            for j in (i + 1)..participant_count {
                let node = BracketNode {
                    id: format!("rr_match_{}", match_number),
                    round: 1, // All matches are considered round 1 for round robin
                    match_id: None,
                    participant1_id: Some(participants[i].0),
                    participant1_name: Some(participants[i].1.clone()),
                    participant2_id: Some(participants[j].0),
                    participant2_name: Some(participants[j].1.clone()),
                    winner_id: None,
                    next_match_id: None,
                    position: match_number,
                };
                matches.push(node);
                match_number += 1;
            }
        }

        Ok((json!({ "matches": matches }), total_rounds as i32))
    }

    async fn create_matches_from_bracket(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
        bracket_data: &JsonValue,
    ) -> Result<Vec<BracketMatch>, sqlx::Error> {
        let mut bracket_matches = Vec::new();

        // Handle single elimination nodes
        if let Some(nodes) = bracket_data.get("nodes").and_then(|n| n.as_array()) {
            for node in nodes {
                if let Some(bracket_match) = Self::create_match_from_node(
                    tx,
                    tournament_id,
                    category_id,
                    node,
                )
                .await?
                {
                    bracket_matches.push(bracket_match);
                }
            }
        }

        // Handle round robin matches
        if let Some(matches) = bracket_data.get("matches").and_then(|m| m.as_array()) {
            for match_node in matches {
                if let Some(bracket_match) = Self::create_match_from_node(
                    tx,
                    tournament_id,
                    category_id,
                    match_node,
                )
                .await?
                {
                    bracket_matches.push(bracket_match);
                }
            }
        }

        Ok(bracket_matches)
    }

    async fn create_match_from_node(
        _tx: &mut sqlx::PgConnection,
        _tournament_id: Uuid,
        _category_id: Option<Uuid>,
        node: &JsonValue,
    ) -> Result<Option<BracketMatch>, sqlx::Error> {
        let participant1_id = node.get("participant1_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());
        let participant2_id = node.get("participant2_id")
            .and_then(|v| v.as_str())
            .and_then(|s| Uuid::parse_str(s).ok());

        // Only create match if we have at least one actual participant (not TBD)
        if participant1_id.is_none() && participant2_id.is_none() {
            return Ok(None);
        }

        let round = node.get("round").and_then(|r| r.as_i64()).unwrap_or(1) as i32;
        let position = node.get("position").and_then(|p| p.as_i64()).unwrap_or(1) as i32;

        // Determine match type based on round and bracket structure
        let match_type = Self::determine_match_type(round, position);

        let new_match = NewMatch {
            tournament_category_id: _category_id.unwrap_or(_tournament_id), // Use tournament_id if no category
            participant1_team_id: participant1_id,
            participant1_player_id: None,
            participant1_partner_id: None,
            participant2_team_id: participant2_id,
            participant2_player_id: None,
            participant2_partner_id: None,
            match_type,
            round_number: Some(round),
            match_number: Some(position),
            scheduled_date: chrono::Utc::now() + chrono::Duration::days(1), // Default to tomorrow
            venue: None,
            court_number: None,
            referee_name: None,
            umpire_name: None,
            notes: None,
            metadata: Some(json!({
                "bracket_node_id": node.get("id").and_then(|v| v.as_str()).unwrap_or(""),
                "generated": true
            })),
        };

        // TODO: Fix match creation when MatchRepository is compatible
        let created_match_id = Uuid::new_v4();

        Ok(Some(BracketMatch {
            id: created_match_id,
            bracket_node_id: node.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            round,
            position,
            participant1_id,
            participant1_name: node.get("participant1_name").and_then(|v| v.as_str()).map(String::from),
            participant2_id,
            participant2_name: node.get("participant2_name").and_then(|v| v.as_str()).map(String::from),
            winner_id: None,
            match_status: "scheduled".to_string(),
            scheduled_date: Some(chrono::Utc::now() + chrono::Duration::days(1)),
        }))
    }

    fn determine_match_type(round: i32, _position: i32) -> MatchType {
        match round {
            1 => MatchType::RoundOf64, // Could be refined based on total participants
            2 => MatchType::RoundOf32,
            3 => MatchType::RoundOf16,
            4 => MatchType::QuarterFinal,
            5 => MatchType::SemiFinal,
            6 => MatchType::Final,
            _ => MatchType::Qualifying,
        }
    }

    async fn get_bracket_details(
        pool: &DbPool,
        bracket_id: Uuid,
    ) -> Result<BracketResponse, sqlx::Error> {
        with_transaction(pool, |tx| {
            Box::pin(async move {
                let bracket = TournamentBracketRepository::get_by_id(tx, bracket_id).await?;
                
                // Get matches for this bracket
                let matches = Self::get_bracket_matches(tx, bracket.tournament_id, bracket.category_id).await?;
                
                // Get participants
                let participants = Self::get_bracket_participants(tx, bracket.tournament_id, bracket.category_id).await?;

                Ok(BracketResponse {
                    bracket,
                    matches,
                    participants,
                })
            })
        })
        .await
    }

    async fn get_bracket_matches(
        _tx: &mut sqlx::PgConnection,
        _tournament_id: Uuid,
        _category_id: Option<Uuid>,
    ) -> Result<Vec<BracketMatch>, sqlx::Error> {
        // TODO: Implement with proper sea-query when matches table exists
        // For now, return empty vector for compilation
        Ok(vec![])
    }

    async fn get_bracket_participants(
        tx: &mut sqlx::PgConnection,
        tournament_id: Uuid,
        category_id: Option<Uuid>,
    ) -> Result<Vec<BracketParticipant>, sqlx::Error> {
        let participants = Self::get_participants(tx, tournament_id, category_id).await?;

        let bracket_participants = participants
            .into_iter()
            .enumerate()
            .map(|(index, (id, name))| BracketParticipant {
                id,
                name,
                seed: Some(index as i32 + 1),
                eliminated: false,
                current_round: Some(1),
            })
            .collect();

        Ok(bracket_participants)
    }
}