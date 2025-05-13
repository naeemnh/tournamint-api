use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool,
    formatters,
    models::player::{CreatePlayer, EditablePlayer},
    repositories::player_repository,
};

pub async fn get_all_players(pool: &DbPool) -> HttpResponse {
    match player_repository::find_all(pool).await {
        Ok(players) => formatters::success_response(StatusCode::OK, players, "RETURNED_PLAYERS"),
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

pub async fn create_player(pool: &DbPool, player_data: CreatePlayer) -> HttpResponse {
    match player_repository::create(pool, player_data).await {
        Ok(player) => formatters::success_response(StatusCode::CREATED, player, "PLAYER_CREATED"),
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

pub async fn get_player(pool: &DbPool, id: Uuid) -> HttpResponse {
    match player_repository::find_by_id(pool, id).await {
        Ok(Some(player)) => formatters::success_response(StatusCode::OK, player, "PLAYER_FOUND"),
        Ok(None) => formatters::error_response(
            StatusCode::NOT_FOUND,
            "Could not find player",
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

pub async fn update_player(pool: &DbPool, id: Uuid, player_data: EditablePlayer) -> HttpResponse {
    match player_repository::update(pool, id, player_data).await {
        Ok(Some(player)) => formatters::success_response(StatusCode::OK, player, "PLAYER_UPDATED"),
        Ok(None) => formatters::error_response(
            StatusCode::BAD_REQUEST,
            "PLAYER_NOT_FOUND",
            "Could Not Find Player",
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

pub async fn delete_player(pool: &DbPool, id: Uuid) -> HttpResponse {
    match player_repository::delete(pool, id).await {
        Ok(Some(player)) => formatters::success_response(StatusCode::OK, player, "PLAYER_DELETED"),
        Ok(None) => formatters::error_response(
            StatusCode::BAD_REQUEST,
            "PLAYER_NOT_FOUND",
            "Could Not Find Player",
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
