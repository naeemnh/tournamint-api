use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::TournamentServices;
use crate::domain::tournament::{
    EditableTournament, EditableTournamentCategory, EditableTournamentRegistration, NewTournament,
    NewTournamentCategory, NewTournamentRegistration, TournamentSearchQuery, TournamentStatus,
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

type TournamentServicesData = std::sync::Arc<
    TournamentServices<
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
    pub async fn index(services: web::Data<TournamentServicesData>) -> HttpResponse {
        match services.get_all_tournaments().await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_status(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentStatusPath>,
    ) -> HttpResponse {
        match services.get_tournaments_by_status(path.status).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_my(
        services: web::Data<TournamentServicesData>,
        req: HttpRequest,
    ) -> HttpResponse {
        let user_id = match get_user_id_from_request(&req) {
            Ok(id) => id,
            Err(response) => return response,
        };
        match services.get_my_tournaments(user_id).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn post(
        services: web::Data<TournamentServicesData>,
        body: web::Json<NewTournament>,
    ) -> HttpResponse {
        match services.create_tournament(body.into_inner()).await {
            Ok(tournament) => ApiResponse::created("Created", tournament),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.get_tournament_by_id(id).await {
            Ok(Some(tournament)) => ApiResponse::success("OK", Some(tournament)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTournament>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.update_tournament(id, body.into_inner()).await {
            Ok(Some(tournament)) => ApiResponse::success("Updated", Some(tournament)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.delete_tournament(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn create_category(
        services: web::Data<TournamentServicesData>,
        body: web::Json<NewTournamentCategory>,
    ) -> HttpResponse {
        match services.create_category(body.into_inner()).await {
            Ok(category) => ApiResponse::created("Created", category),
            Err(e) => e.error_response(),
        }
    }

    pub async fn create_registration(
        services: web::Data<TournamentServicesData>,
        body: web::Json<NewTournamentRegistration>,
    ) -> HttpResponse {
        match services.create_registration(body.into_inner()).await {
            Ok(registration) => ApiResponse::created("Created", registration),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_brackets_by_tournament(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        let tournament_id = path.tournament_id;
        match services.get_brackets_by_tournament(tournament_id).await {
            Ok(brackets) => ApiResponse::success("OK", Some(brackets)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_standings_by_tournament(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        let tournament_id = path.tournament_id;
        match services.get_standings_by_tournament(tournament_id).await {
            Ok(standings) => ApiResponse::success("OK", Some(standings)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn search(
        services: web::Data<TournamentServicesData>,
        query: web::Query<TournamentSearchQuery>,
    ) -> HttpResponse {
        match services.search_tournaments(query.into_inner()).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_featured(
        services: web::Data<TournamentServicesData>,
        query: web::Query<FeaturedLimitQuery>,
    ) -> HttpResponse {
        let limit = query.limit.unwrap_or(10).min(100);
        match services.get_featured_tournaments(limit).await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_upcoming(services: web::Data<TournamentServicesData>) -> HttpResponse {
        match services.get_upcoming_tournaments().await {
            Ok(tournaments) => ApiResponse::success("OK", Some(tournaments)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn publish(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.publish_tournament(id).await {
            Ok(Some(t)) => ApiResponse::success("Published", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn start(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.start_tournament(id).await {
            Ok(Some(t)) => ApiResponse::success("Started", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn complete(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.complete_tournament(id).await {
            Ok(Some(t)) => ApiResponse::success("Completed", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn cancel(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: Option<web::Json<CancelTournamentBody>>,
    ) -> HttpResponse {
        let id = path.into_inner();
        let reason = body.and_then(|b| b.reason.clone());
        match services.cancel_tournament(id, reason).await {
            Ok(Some(t)) => ApiResponse::success("Cancelled", Some(t)),
            Ok(None) => ApiResponse::not_found("Tournament not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_stats(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.get_tournament_stats(id).await {
            Ok(stats) => ApiResponse::success("OK", Some(stats)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_participants(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.get_tournament_participants(id).await {
            Ok(participants) => ApiResponse::success("OK", Some(participants)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn export(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        query: web::Query<ExportFormatQuery>,
    ) -> HttpResponse {
        let id = path.into_inner();
        let format = query.format.clone().unwrap_or_else(|| "json".to_string());
        match services.export_tournament(id, format).await {
            Ok(data) => ApiResponse::success("OK", Some(data)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn duplicate(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<DuplicateTournamentBody>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.duplicate_tournament(id, body.name.clone()).await {
            Ok(tournament) => ApiResponse::created("Duplicated", tournament),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_templates(services: web::Data<TournamentServicesData>) -> HttpResponse {
        match services.get_tournament_templates().await {
            Ok(templates) => ApiResponse::success("OK", Some(templates)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn create_from_template(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<NewTournament>,
    ) -> HttpResponse {
        let template_id = path.into_inner();
        match services
            .create_from_template(template_id, body.into_inner())
            .await
        {
            Ok(tournament) => ApiResponse::created("Created from template", tournament),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_dashboard(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.get_tournament_dashboard(id).await {
            Ok(dashboard) => ApiResponse::success("OK", Some(dashboard)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_settings(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<serde_json::Value>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services
            .update_tournament_settings(id, body.into_inner())
            .await
        {
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

/// Tournament category handlers - delegate to TournamentHandler with services
pub struct TournamentCategoryHandler;

impl TournamentCategoryHandler {
    pub async fn create_category(
        services: web::Data<TournamentServicesData>,
        body: web::Json<NewTournamentCategory>,
    ) -> HttpResponse {
        TournamentHandler::create_category(services, body).await
    }

    pub async fn get_by_id(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.get_category_by_id(id).await {
            Ok(Some(category)) => ApiResponse::success("OK", Some(category)),
            Ok(None) => ApiResponse::not_found("Category not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_tournament(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match services
            .get_categories_by_tournament(path.tournament_id)
            .await
        {
            Ok(categories) => ApiResponse::success("OK", Some(categories)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTournamentCategory>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.update_category(id, body.into_inner()).await {
            Ok(Some(category)) => ApiResponse::success("Updated", Some(category)),
            Ok(None) => ApiResponse::not_found("Category not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.delete_category(id).await {
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
        services: web::Data<TournamentServicesData>,
        body: web::Json<NewTournamentRegistration>,
    ) -> HttpResponse {
        TournamentHandler::create_registration(services, body).await
    }

    pub async fn get_by_id(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.get_registration_by_id(id).await {
            Ok(Some(reg)) => ApiResponse::success("OK", Some(reg)),
            Ok(None) => ApiResponse::not_found("Registration not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTournamentRegistration>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.update_registration(id, body.into_inner()).await {
            Ok(Some(reg)) => ApiResponse::success("Updated", Some(reg)),
            Ok(None) => ApiResponse::not_found("Registration not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        services: web::Data<TournamentServicesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match services.delete_registration(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Registration not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_category(
        services: web::Data<TournamentServicesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match services
            .get_registrations_by_category(path.category_id)
            .await
        {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_tournament(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match services
            .get_registrations_by_tournament(path.tournament_id)
            .await
        {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_player(
        services: web::Data<TournamentServicesData>,
        path: web::Path<PlayerIdPath>,
    ) -> HttpResponse {
        match services.get_registrations_by_player(path.player_id).await {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_team(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TeamIdPath>,
    ) -> HttpResponse {
        match services.get_registrations_by_team(path.team_id).await {
            Ok(regs) => ApiResponse::success("OK", Some(regs)),
            Err(e) => e.error_response(),
        }
    }
}

/// Tournament bracket handlers
pub struct TournamentBracketHandler;

impl TournamentBracketHandler {
    pub async fn get_brackets_by_tournament(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        TournamentHandler::get_brackets_by_tournament(services, path).await
    }

    pub async fn get_by_category(
        services: web::Data<TournamentServicesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match services.get_bracket_by_category(path.category_id).await {
            Ok(Some(bracket)) => ApiResponse::success("OK", Some(bracket)),
            Ok(None) => ApiResponse::not_found("Bracket not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn generate(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match services.generate_bracket(path.tournament_id).await {
            Ok(bracket) => ApiResponse::success("Generated", Some(bracket)),
            Err(e) => e.error_response(),
        }
    }
}

/// Tournament standings handlers
pub struct TournamentStandingsHandler;

impl TournamentStandingsHandler {
    pub async fn get_standings_by_tournament(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        TournamentHandler::get_standings_by_tournament(services, path).await
    }

    pub async fn get_by_category(
        services: web::Data<TournamentServicesData>,
        path: web::Path<CategoryIdPath>,
    ) -> HttpResponse {
        match services.get_standings_by_category(path.category_id).await {
            Ok(standings) => ApiResponse::success("OK", Some(standings)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update_standings(
        services: web::Data<TournamentServicesData>,
        path: web::Path<TournamentIdPath>,
    ) -> HttpResponse {
        match services.recalculate_standings(path.tournament_id).await {
            Ok(count) => {
                ApiResponse::success("Updated", Some(serde_json::json!({ "deleted": count })))
            }
            Err(e) => e.error_response(),
        }
    }
}
