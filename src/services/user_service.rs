use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::config::DbPool;
use crate::constants::errors::{DUPLICATE_USER_EMAIL, DUPLICATE_USER_USERNAME};
use crate::formatters;
use crate::models::user::{EditableUser, NewUser};
use crate::repositories::UserRepository;
use crate::utils::db::with_transaction;

pub struct UserService;

impl UserService {
    pub async fn get_all_users(pool: &DbPool) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserRepository::find_all(tx).await })
        })
        .await
        {
            Ok(users) => formatters::success_response(StatusCode::OK, users, "RETURNED_USERS"),
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

    pub async fn create_user(pool: &DbPool, user_data: NewUser) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserRepository::create(tx, user_data).await })
        })
        .await
        {
            Ok(user) => formatters::success_response(StatusCode::CREATED, user, "USER_CREATED"),
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    DUPLICATE_USER_EMAIL => "User is already registered",
                    DUPLICATE_USER_USERNAME => "username is already taken",
                    err => err,
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "USER_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_user(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserRepository::find_by_id(tx, id).await })
        })
        .await
        {
            Ok(Some(user)) => formatters::success_response(StatusCode::OK, user, "USER_FOUND"),
            Ok(None) => formatters::error_response(
                StatusCode::NOT_FOUND,
                "Could not find user",
                "USER_NOT_FOUND",
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

    pub async fn update_user(pool: &DbPool, id: Uuid, user_data: EditableUser) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserRepository::update(tx, id, user_data).await })
        })
        .await
        {
            Ok(Some(user)) => formatters::success_response(StatusCode::OK, user, "USER_UPDATED"),
            Ok(None) => formatters::error_response(
                StatusCode::BAD_REQUEST,
                "USER_NOT_FOUND",
                "Could Not Find User",
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

    pub async fn delete_user(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { UserRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(Some(user)) => formatters::success_response(StatusCode::OK, user, "USER_DELETED"),
            Ok(None) => formatters::error_response(
                StatusCode::BAD_REQUEST,
                "USER_NOT_FOUND",
                "Could Not Find User",
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
