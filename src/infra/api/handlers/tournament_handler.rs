use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::TournamentUseCases;
use crate::domain::tournament::{
    EditableTournament, NewTournament, NewTournamentCategory, NewTournamentRegistration,
};
use crate::infra::db::{
    PgTournamentBracketRepository, PgTournamentCategoryRepository,
    PgTournamentRegistrationRepository, PgTournamentRepository, PgTournamentStandingsRepository,
};
use crate::shared::ApiResponse;

#[derive(Deserialize)]
pub struct TournamentIdPath {
    tournament_id: Uuid,
}

type TournamentUseCasesData = std::sync::Arc<
    TournamentUseCases<
        PgTournamentRepository,
        PgTournamentCategoryRepository,
        PgTournamentRegistrationRepository,
        PgTournamentBracketRepository,
        PgTournamentStandingsRepository,
    >,
>;

pub struct TournamentHandler;

impl TournamentHandler {
    pub async fn index(use_cases: web::Data<TournamentUseCasesData>) -> HttpResponse {
        match use_cases.get_all_tournaments().await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn post(
        use_cases: web::Data<TournamentUseCasesData>,
        body: web::Json<NewTournament>,
    ) -> HttpResponse {
        match use_cases.create_tournament(body.into_inner()).await {
            Ok(tournament) => ApiResponse::created("Created", tournament),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_tournament_by_id(id).await {
            Ok(Some(tournament)) => ApiResponse::success("OK", Some(tournament)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTournament>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_tournament(id, body.into_inner()).await {
            Ok(Some(tournament)) => ApiResponse::success("Updated", Some(tournament)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_tournament(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn create_category(
        use_cases: web::Data<TournamentUseCasesData>,
        body: web::Json<NewTournamentCategory>,
    ) -> HttpResponse {
        match use_cases.create_category(body.into_inner()).await {
            Ok(category) => ApiResponse::created("Created", category),
            Err(e) => e.error_response(),
        }
    }

    pub async fn create_registration(
        use_cases: web::Data<TournamentUseCasesData>,
        body: web::Json<NewTournamentRegistration>,
    ) -> HttpResponse {
        match use_cases.create_registration(body.into_inner()).await {
            Ok(registration) => ApiResponse::created("Created", registration),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_brackets_by_tournament(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        let tournament_id = path.tournament_id;
        match use_cases.get_brackets_by_tournament(tournament_id).await {
            Ok(brackets) => ApiResponse::success("OK", Some(brackets)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_standings_by_tournament(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        let tournament_id = path.tournament_id;
        match use_cases.get_standings_by_tournament(tournament_id).await {
            Ok(standings) => ApiResponse::success("OK", Some(standings)),
            Err(e) => e.error_response(),
        }
    }
}

/// Tournament category handlers - delegate to TournamentHandler with use cases
pub struct TournamentCategoryHandler;

impl TournamentCategoryHandler {
    pub async fn create_category(
        use_cases: web::Data<TournamentUseCasesData>,
        body: web::Json<NewTournamentCategory>,
    ) -> HttpResponse {
        TournamentHandler::create_category(use_cases, body).await
    }
}

/// Tournament registration handlers
pub struct TournamentRegistrationHandler;

impl TournamentRegistrationHandler {
    pub async fn create_registration(
        use_cases: web::Data<TournamentUseCasesData>,
        body: web::Json<NewTournamentRegistration>,
    ) -> HttpResponse {
        TournamentHandler::create_registration(use_cases, body).await
    }
}

/// Tournament bracket handlers
pub struct TournamentBracketHandler;

impl TournamentBracketHandler {
    pub async fn get_brackets_by_tournament(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        TournamentHandler::get_brackets_by_tournament(use_cases, path).await
    }
}

/// Tournament standings handlers
pub struct TournamentStandingsHandler;

impl TournamentStandingsHandler {
    pub async fn get_standings_by_tournament(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        TournamentHandler::get_standings_by_tournament(use_cases, path).await
    }
}
