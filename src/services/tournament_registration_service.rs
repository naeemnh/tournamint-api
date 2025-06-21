use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::tournament_registration::{EditableTournamentRegistration, NewTournamentRegistration},
    repositories::TournamentRegistrationRepository,
    utils::db::with_transaction,
};

pub struct TournamentRegistrationService;

impl TournamentRegistrationService {
    pub async fn create_registration(
        pool: &DbPool,
        data: NewTournamentRegistration,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRegistrationRepository::create(tx, data).await })
        })
        .await
        {
            Ok(registration) => {
                formatters::success_response(StatusCode::CREATED, registration, "REGISTRATION_CREATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("duplicate key") {
                    "Already registered for this tournament category"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "REGISTRATION_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_registration_by_id(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRegistrationRepository::get_by_id(tx, id).await })
        })
        .await
        {
            Ok(registration) => {
                formatters::success_response(StatusCode::OK, registration, "REGISTRATION_FETCHED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Registration not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "REGISTRATION_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_registrations_by_tournament_category(
        pool: &DbPool,
        tournament_category_id: Uuid,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRegistrationRepository::get_by_tournament_category(tx, tournament_category_id)
                    .await
            })
        })
        .await
        {
            Ok(registrations) => {
                formatters::success_response(StatusCode::OK, registrations, "REGISTRATIONS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "REGISTRATIONS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_registrations_by_tournament(
        pool: &DbPool,
        tournament_id: Uuid,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRegistrationRepository::get_by_tournament(tx, tournament_id).await
            })
        })
        .await
        {
            Ok(registrations) => {
                formatters::success_response(StatusCode::OK, registrations, "REGISTRATIONS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "REGISTRATIONS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_registrations_by_player(pool: &DbPool, player_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRegistrationRepository::get_by_player(tx, player_id).await
            })
        })
        .await
        {
            Ok(registrations) => {
                formatters::success_response(StatusCode::OK, registrations, "REGISTRATIONS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "REGISTRATIONS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_registrations_by_team(pool: &DbPool, team_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentRegistrationRepository::get_by_team(tx, team_id).await
            })
        })
        .await
        {
            Ok(registrations) => {
                formatters::success_response(StatusCode::OK, registrations, "REGISTRATIONS_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "REGISTRATIONS_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn update_registration(
        pool: &DbPool,
        id: Uuid,
        data: EditableTournamentRegistration,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRegistrationRepository::update(tx, id, data).await })
        })
        .await
        {
            Ok(registration) => {
                formatters::success_response(StatusCode::OK, registration, "REGISTRATION_UPDATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Registration not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "REGISTRATION_UPDATE_ERROR",
                )
            }
        }
    }

    pub async fn delete_registration(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentRegistrationRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(registration) => {
                formatters::success_response(StatusCode::OK, registration, "REGISTRATION_DELETED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Registration not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "REGISTRATION_DELETE_ERROR",
                )
            }
        }
    }
}