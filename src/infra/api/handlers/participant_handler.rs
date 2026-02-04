use actix_web::{web, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::application::ParticipantUseCases;
use crate::domain::participant::{
    CreatePlayer, EditablePlayer, EditableTeam, EditableTeamMember, NewTeam, NewTeamMember,
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

#[derive(Debug, Deserialize)]
pub struct TeamMemberTeamIdPath {
    pub team_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TeamMemberPlayerIdPath {
    pub player_id: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct TeamMemberCompositePath {
    pub team_id: Uuid,
    pub player_id: Uuid,
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

    pub async fn get_by_team(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<TeamMemberTeamIdPath>,
    ) -> HttpResponse {
        match use_cases.get_team_members(path.team_id).await {
            Ok(members) => ApiResponse::success("OK", Some(members)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn get_by_player(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<TeamMemberPlayerIdPath>,
    ) -> HttpResponse {
        match use_cases.get_team_members_by_player(path.player_id).await {
            Ok(members) => ApiResponse::success("OK", Some(members)),
            Err(e) => e.error_response(),
        }
    }

    pub async fn update(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<TeamMemberCompositePath>,
        body: web::Json<EditableTeamMember>,
    ) -> HttpResponse {
        match use_cases
            .update_team_member(path.team_id, path.player_id, body.into_inner())
            .await
        {
            Ok(Some(member)) => ApiResponse::success("Updated", Some(member)),
            Ok(None) => ApiResponse::not_found("Team member not found"),
            Err(e) => e.error_response(),
        }
    }

    pub async fn delete(
        use_cases: web::Data<ParticipantUseCasesData>,
        path: web::Path<TeamMemberCompositePath>,
    ) -> HttpResponse {
        match use_cases.remove_team_member(path.team_id, path.player_id).await {
            Ok(Some(_)) => ApiResponse::success("Deleted", Some(serde_json::json!({}))),
            Ok(None) => ApiResponse::not_found("Team member not found"),
            Err(e) => e.error_response(),
        }
    }
}
