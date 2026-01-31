use actix_web::{web, HttpResponse, ResponseError};
use uuid::Uuid;

use crate::application::ParticipantUseCases;
use crate::domain::participant::{
    CreatePlayer, EditablePlayer, EditableTeam, NewTeam, NewTeamMember,
};
use crate::infra::db::{PgPlayerRepository, PgTeamMemberRepository, PgTeamRepository};
use crate::shared::ApiResponse;

type ParticipantUseCasesData = std::sync::Arc<
    ParticipantUseCases<PgPlayerRepository, PgTeamRepository, PgTeamMemberRepository>,
>;

pub struct PlayerHandler;

impl PlayerHandler {
    pub async fn index(use_cases: web::Data<ParticipantUseCasesData>) -> HttpResponse {
        match use_cases.get_all_players().await {
            Ok(players) => ApiResponse::success("OK", Some(players)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn post(
        use_cases: web::Data<ParticipantUseCasesData>,
        body: web::Json<CreatePlayer>,
    ) -> HttpResponse {
        match use_cases.create_player(body.into_inner()).await {
            Ok(player) => ApiResponse::created("Created", player),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_player(id).await {
            Ok(Some(player)) => ApiResponse::success("OK", Some(player)),
            Ok(None) => ApiResponse::not_found("Player not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditablePlayer>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_player(id, body.into_inner()).await {
            Ok(Some(player)) => ApiResponse::success("Updated", Some(player)),
            Ok(None) => ApiResponse::not_found("Player not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_player(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Player not found"),
            Err(e) => e.error_response(),
        }
    }
}

pub struct TeamHandler;

impl TeamHandler {
    pub async fn index(use_cases: web::Data<ParticipantUseCasesData>) -> HttpResponse {
        match use_cases.get_all_teams().await {
            Ok(teams) => ApiResponse::success("OK", Some(teams)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn post(
        use_cases: web::Data<ParticipantUseCasesData>,
        body: web::Json<NewTeam>,
    ) -> HttpResponse {
        match use_cases.create_team(body.into_inner()).await {
            Ok(team) => ApiResponse::created("Created", team),
            Err(e) => e.error_response(),
        }
    }

    pub async fn show(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.get_team(id).await {
            Ok(Some(team)) => ApiResponse::success("OK", Some(team)),
            Ok(None) => ApiResponse::not_found("Team not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<Uuid>,
        body: web::Json<EditableTeam>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.update_team(id, body.into_inner()).await {
            Ok(Some(team)) => ApiResponse::success("Updated", Some(team)),
            Ok(None) => ApiResponse::not_found("Team not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<Uuid>,
    ) -> HttpResponse {
        let id = path.into_inner();
        match use_cases.delete_team(id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Team not found"),
            Err(e) => e.error_response(),
        }
    }
}

pub struct TeamMemberHandler;

impl TeamMemberHandler {
    pub async fn post(
        use_cases: web::Data<ParticipantUseCasesData>,
        body: web::Json<NewTeamMember>,
    ) -> HttpResponse {
        match use_cases.add_team_member(body.into_inner()).await {
            Ok(member) => ApiResponse::created("Created", member),
            Err(e) => e.error_response(),
        }
    }
}
