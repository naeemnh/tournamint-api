use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool, formatters,
    models::tournament_category::{EditableTournamentCategory, NewTournamentCategory},
    repositories::TournamentCategoryRepository, utils::db::with_transaction,
};

pub struct TournamentCategoryService;

impl TournamentCategoryService {
    pub async fn create_category(pool: &DbPool, data: NewTournamentCategory) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentCategoryRepository::create(tx, data).await })
        })
        .await
        {
            Ok(category) => {
                formatters::success_response(StatusCode::CREATED, category, "CATEGORY_CREATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("duplicate key") {
                    "Category with this name already exists for this tournament"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "CATEGORY_CREATION_ERROR",
                )
            }
        }
    }

    pub async fn get_categories_by_tournament(pool: &DbPool, tournament_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TournamentCategoryRepository::get_by_tournament(tx, tournament_id).await
            })
        })
        .await
        {
            Ok(categories) => {
                formatters::success_response(StatusCode::OK, categories, "CATEGORIES_FETCHED")
            }
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "CATEGORIES_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn get_category_by_id(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentCategoryRepository::get_by_id(tx, id).await })
        })
        .await
        {
            Ok(category) => {
                formatters::success_response(StatusCode::OK, category, "CATEGORY_FETCHED")
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
                    "CATEGORY_FETCH_ERROR",
                )
            }
        }
    }

    pub async fn update_category(
        pool: &DbPool,
        id: Uuid,
        data: EditableTournamentCategory,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentCategoryRepository::update(tx, id, data).await })
        })
        .await
        {
            Ok(category) => {
                formatters::success_response(StatusCode::OK, category, "CATEGORY_UPDATED")
            }
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Category not found"
                } else if e.to_string().contains("duplicate key") {
                    "Category with this name already exists for this tournament"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "CATEGORY_UPDATE_ERROR",
                )
            }
        }
    }

    pub async fn delete_category(pool: &DbPool, id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TournamentCategoryRepository::delete(tx, id).await })
        })
        .await
        {
            Ok(category) => {
                formatters::success_response(StatusCode::OK, category, "CATEGORY_DELETED")
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
                    "CATEGORY_DELETE_ERROR",
                )
            }
        }
    }
}