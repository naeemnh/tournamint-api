use actix_web::{web, HttpRequest, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tournament::{EditableTournament, NewTournament, TournamentStatus},
    services::TournamentService,
};

#[derive(Debug, Deserialize)]
pub struct TournamentSearchQuery {
    pub name: Option<String>,
    pub sport_type: Option<String>,
    pub status: Option<String>,
    #[allow(dead_code)]
    pub format: Option<String>,
    pub location: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>, // json, csv, pdf
}

#[derive(Debug, Deserialize)]
pub struct DuplicateRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CancelRequest {
    pub reason: Option<String>,
}

pub struct TournamentController;

impl TournamentController {
    // Basic CRUD operations
    pub async fn create(pool: web::Data<DbPool>, data: web::Json<NewTournament>) -> impl Responder {
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

    // Search and filtering
    pub async fn search(
        pool: web::Data<DbPool>,
        query: web::Query<TournamentSearchQuery>,
    ) -> impl Responder {
        TournamentService::search_tournaments(&pool, query.into_inner()).await
    }

    pub async fn get_my_tournaments(pool: web::Data<DbPool>, req: HttpRequest) -> impl Responder {
        // Extract user ID from JWT token in request
        TournamentService::get_my_tournaments(&pool, req).await
    }

    pub async fn get_featured(pool: web::Data<DbPool>) -> impl Responder {
        TournamentService::get_featured_tournaments(&pool).await
    }

    pub async fn get_upcoming(pool: web::Data<DbPool>) -> impl Responder {
        TournamentService::get_upcoming_tournaments(&pool).await
    }

    // Tournament management
    pub async fn publish(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::publish_tournament(&pool, id.into_inner()).await
    }

    pub async fn start(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::start_tournament(&pool, id.into_inner()).await
    }

    pub async fn complete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::complete_tournament(&pool, id.into_inner()).await
    }

    pub async fn cancel(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        data: web::Json<CancelRequest>,
    ) -> impl Responder {
        TournamentService::cancel_tournament(&pool, id.into_inner(), data.reason.clone()).await
    }

    // Statistics and participants
    pub async fn get_stats(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::get_tournament_stats(&pool, id.into_inner()).await
    }

    pub async fn get_participants(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::get_tournament_participants(&pool, id.into_inner()).await
    }

    // Export and duplicate
    pub async fn export(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        query: web::Query<ExportQuery>,
    ) -> impl Responder {
        let format = query.format.clone().unwrap_or_else(|| "json".to_string());
        TournamentService::export_tournament(&pool, id.into_inner(), format).await
    }

    pub async fn duplicate(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        data: web::Json<DuplicateRequest>,
    ) -> impl Responder {
        TournamentService::duplicate_tournament(&pool, id.into_inner(), data.name.clone()).await
    }

    // Templates
    pub async fn get_templates(pool: web::Data<DbPool>) -> impl Responder {
        TournamentService::get_tournament_templates(&pool).await
    }

    pub async fn create_from_template(
        pool: web::Data<DbPool>,
        template_id: web::Path<Uuid>,
        data: web::Json<NewTournament>,
    ) -> impl Responder {
        TournamentService::create_from_template(&pool, template_id.into_inner(), data.into_inner())
            .await
    }

    // Organizer functions
    pub async fn get_dashboard(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::get_tournament_dashboard(&pool, id.into_inner()).await
    }

    pub async fn update_settings(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        data: web::Json<serde_json::Value>,
    ) -> impl Responder {
        TournamentService::update_tournament_settings(&pool, id.into_inner(), data.into_inner())
            .await
    }

    // Sub-resources
    pub async fn get_categories(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::get_tournament_categories(&pool, id.into_inner()).await
    }

    pub async fn get_registrations(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentService::get_tournament_registrations(&pool, id.into_inner()).await
    }
}
