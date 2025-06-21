use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tournament::{EditableTournament, NewTournament, TournamentStatus},
    services::TournamentService,
};

pub struct TournamentController;

impl TournamentController {
    pub async fn create(
        pool: web::Data<DbPool>,
        data: web::Json<NewTournament>,
    ) -> impl Responder {
        TournamentService::create_tournament(&pool, data.into_inner()).await
    }

    pub async fn get_all(pool: web::Data<DbPool>) -> impl Responder {
        TournamentService::get_all_tournaments(&pool).await
    }

    pub async fn get_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::get_tournament_by_id(&pool, id.into_inner()).await
    }

    pub async fn get_by_status(
        pool: web::Data<DbPool>,
        status: web::Path<String>,
    ) -> impl Responder {
        match serde_json::from_str::<TournamentStatus>(&format!("\"{}\"", status.into_inner())) {
            Ok(tournament_status) => {
                TournamentService::get_tournaments_by_status(&pool, tournament_status).await
            }
            Err(_) => {
                use crate::formatters;
                use oauth2::http::StatusCode;
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    "Invalid tournament status",
                    "INVALID_STATUS",
                )
            }
        }
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        data: web::Json<EditableTournament>,
    ) -> impl Responder {
        TournamentService::update_tournament(&pool, id.into_inner(), data.into_inner()).await
    }

    pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::delete_tournament(&pool, id.into_inner()).await
    }
}