use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::TournamentUseCases;
use crate::domain::tournament::{
    EditableTournament, NewTournament, NewTournamentCategory, NewTournamentRegistration,
    TournamentSearchQuery,
};
use crate::infra::db::{
    PgTournamentBracketRepository, PgTournamentCategoryRepository,
    PgTournamentRegistrationRepository, PgTournamentRepository, PgTournamentStandingsRepository,
};
use crate::shared::ApiResponse;

#[derive(Deserialize)]
pub struct TournamentIdPath {
    pub tournament_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CancelTournamentBody {
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DuplicateTournamentBody {
    pub name: String,
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

    pub async fn search(
        use_cases: web::Data<TournamentUseCasesData>,
        query: web::Query<TournamentSearchQuery>,
    ) -> HttpResponse {
        match use_cases.search_tournaments(query.into_inner()).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_featured(
        use_cases: web::Data<TournamentUseCasesData>,
        query: web::Query<FeaturedLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(10).min(100);
        match use_cases.get_featured_tournaments(limit).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_upcoming(use_cases: web::Data<TournamentUseCasesData>) -> HttpResponse {
        match use_cases.get_upcoming_tournaments().await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn publish(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.publish_tournament(id).await {
            Ok(Some(t)) => ApiResponse::success("Published", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn start(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.start_tournament(id).await {
            Ok(Some(t)) => ApiResponse::success("Started", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn complete(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.complete_tournament(id).await {
            Ok(Some(t)) => ApiResponse::success("Completed", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn cancel(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: Option<web::Json<CancelTournamentBody>>,
    ) -> HttpResponse {
        let id = path.into_inner();
        let reason = body.and_then(|b| b.reason.clone());
        match use_cases.cancel_tournament(id, reason).await {
            Ok(Some(t)) => ApiResponse::success("Cancelled", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_stats(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_tournament_stats(id).await {
            Ok(stats) => ApiResponse::success("OK", Some(stats)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_participants(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_tournament_participants(id).await {
            Ok(participants) => ApiResponse::success("OK", Some(participants)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn export(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        query: web::Query<ExportFormatQuery>,
    ) -> HttpResponse {
        let id = path.into_inner();
        let format = query.format.clone().unwrap_or_else(|| "json".to_string());
        match use_cases.export_tournament(id, format).await {
            Ok(data) => ApiResponse::success("OK", Some(data)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn duplicate(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<DuplicateTournamentBody>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.duplicate_tournament(id, body.name.clone()).await {
            Ok(tournament) => ApiResponse::created("Duplicated", tournament),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_templates(use_cases: web::Data<TournamentUseCasesData>) -> HttpResponse {
        match use_cases.get_tournament_templates().await {
            Ok(templates) => ApiResponse::success("OK", Some(templates)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn create_from_template(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<NewTournament>,
    ) -> HttpResponse {
        let template_id = path.into_inner();
        match use_cases.create_from_template(template_id, body.into_inner()).await {
            Ok(tournament) => ApiResponse::created("Created from template", tournament),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_dashboard(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_tournament_dashboard(id).await {
            Ok(dashboard) => ApiResponse::success("OK", Some(dashboard)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_settings(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<serde_json::Value>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_tournament_settings(id, body.into_inner()).await {
            Ok(Some(t)) => ApiResponse::success("Settings updated", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct FeaturedLimitQuery {
    pub limit: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ExportFormatQuery {
    pub format: Option<String>,
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
