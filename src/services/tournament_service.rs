use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool, formatters,
    models::tournament::{EditableTournament, NewTournament, TournamentStatus},
    repositories::TournamentRepository, utils::db::with_transaction,
};

pub struct TournamentService;

impl TournamentService {
    pub async fn create_tournament(pool: &DbPool, data: NewTournament) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::create(tx, data).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::CREATED, tournament, "TOURNAMENT_CREATED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    &error_message,
                    "TOURNAMENT_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_all_tournaments(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::get_all(tx).await })
        })
        .await
        {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_tournament_by_id(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::get_by_id(tx, id).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_FETCHED")
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
                    "TOURNAMENT_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_tournaments_by_status(pool: &DbPool, status: TournamentStatus) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::get_by_status(tx, status).await })
        })
        .await
        {
            Ok(tournaments) => {
                formatters::success_response(StatusCode::OK, tournaments, "TOURNAMENTS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "TOURNAMENTS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn update_tournament(
        pool: &DbPool,
        id: Uuid,
        data: EditableTournament,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::update(tx, id, data).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_UPDATED")
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
                    "TOURNAMENT_UPDATE_ERROR",
                )
            }
        }
    }

    pub async fn delete_tournament(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(tournament) => {
                formatters::success_response(StatusCode::OK, tournament, "TOURNAMENT_DELETED")
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
                    "TOURNAMENT_DELETE_ERROR",
                )
            }
        }
    }
}