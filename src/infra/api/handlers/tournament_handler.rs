use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::TournamentUseCases;
use crate::domain::tournament::{
    EditableTournament, EditableTournamentCategory, EditableTournamentRegistration,
    NewTournament, NewTournamentCategory, NewTournamentRegistration, TournamentSearchQuery,
    TournamentStatus,
};
use crate::infra::api::middleware::auth::get_user_id_from_request;
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

#[derive(Debug, Deserialize)]
pub struct TournamentStatusPath {
    pub status: TournamentStatus,
}

impl TournamentHandler {
    pub async fn index(use_cases: web::Data<TournamentUseCasesData>) -> HttpResponse {
        match use_cases.get_all_tournaments().await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_status(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentStatusPath>,
    ) -> HttpResponse {
        match use_cases.get_tournaments_by_status(path.status).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_my(
        use_cases: web::Data<TournamentUseCasesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match use_cases.get_my_tournaments(user_id).await {
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

    pub async fn get_by_id(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_category_by_id(id).await {
            Ok(Some(category)) => ApiResponse::success("OK", Some(category)),
            Ok(None) => ApiResponse::not_found("Category not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_tournament(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases.get_categories_by_tournament(path.tournament_id).await {
            Ok(categories) => ApiResponse::success("OK", Some(categories)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTournamentCategory>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_category(id, body.into_inner()).await {
            Ok(Some(category)) => ApiResponse::success("Updated", Some(category)),
            Ok(None) => ApiResponse::not_found("Category not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_category(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Category not found"),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CategoryIdPath {
    pub category_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct PlayerIdPath {
    pub player_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TeamIdPath {
    pub team_id: Uuid,
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

    pub async fn get_by_id(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_registration_by_id(id).await {
            Ok(Some(reg)) => ApiResponse::success("OK", Some(reg)),
            Ok(None) => ApiResponse::not_found("Registration not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTournamentRegistration>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_registration(id, body.into_inner()).await {
            Ok(Some(reg)) => ApiResponse::success("Updated", Some(reg)),
            Ok(None) => ApiResponse::not_found("Registration not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_registration(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Registration not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_category(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match use_cases.get_registrations_by_category(path.category_id).await {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_tournament(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases.get_registrations_by_tournament(path.tournament_id).await {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_player(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<PlayerIdPath>,
    ) -> HttpResponse {
        match use_cases.get_registrations_by_player(path.player_id).await {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_team(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TeamIdPath>,
    ) -> HttpResponse {
        match use_cases.get_registrations_by_team(path.team_id).await {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
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

    pub async fn get_by_category(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match use_cases.get_bracket_by_category(path.category_id).await {
            Ok(Some(bracket)) => ApiResponse::success("OK", Some(bracket)),
            Ok(None) => ApiResponse::not_found("Bracket not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn generate(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases.generate_bracket(path.tournament_id).await {
            Ok(bracket) => ApiResponse::success("Generated", Some(bracket)),
            Err(e) => e.error_response(),
        }
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

    pub async fn get_by_category(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match use_cases.get_standings_by_category(path.category_id).await {
            Ok(standings) => ApiResponse::success("OK", Some(standings)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_standings(
        use_cases: web::Data<TournamentUseCasesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match use_cases.recalculate_standings(path.tournament_id).await {
            Ok(count) => ApiResponse::success("Updated", Some(serde_json::json!({ "deleted": count }))),
            Err(e) => e.error_response(),
        }
    }
}
