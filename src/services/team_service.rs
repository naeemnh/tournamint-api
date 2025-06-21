use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::team::{EditableTeam, NewTeam},
    repositories::TeamRepository,
    utils::db::with_transaction,
};

pub struct TeamService;

impl TeamService {
    pub async fn get_all_teams(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamRepository::find_all(tx).await })
        })
        .await
        {
            Ok(teams) => formatters::success_response(StatusCode::OK, teams, "RETURNED_PLAYERS"),
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    err => err,
                };
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn create_team(pool: &DbPool, team_data: NewTeam) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamRepository::create(tx, team_data).await })
        })
        .await
        {
            Ok(team) => formatters::success_response(StatusCode::CREATED, team, "PLAYER_CREATED"),
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    err => err,
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "PLAYER_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_team(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamRepository::find_by_id(tx, id).await })
        })
        .await
        {
            Ok(Some(team)) => formatters::success_response(StatusCode::OK, team, "PLAYER_FOUND"),
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Could not find team",
                "PLAYER_NOT_FOUND",
            ),
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    err => err,
                };
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn update_team(pool: &DbPool, id: Uuid, team_data: EditableTeam) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamRepository::update(tx, id, team_data).await })
        })
        .await
        {
            Ok(Some(team)) => formatters::success_response(StatusCode::OK, team, "PLAYER_UPDATED"),
            Ok(None) => formatters::error_response(
                StatusCode::BAD_REQUEST,
                "PLAYER_NOT_FOUND",
                "Could Not Find Team",
            ),
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    err => err,
                };
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }

    pub async fn delete_team(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(Some(team)) => formatters::success_response(StatusCode::OK, team, "PLAYER_DELETED"),
            Ok(None) => formatters::error_response(
                StatusCode::BAD_REQUEST,
                "PLAYER_NOT_FOUND",
                "Could Not Find Team",
            ),
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    err => err,
                };
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    error_message,
                    "UNKNOWN_ERROR",
                )
            }
        }
    }
}
