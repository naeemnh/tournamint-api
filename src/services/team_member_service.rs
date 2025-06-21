use actix_web::HttpResponse;
use oauth2::http::StatusCode;
use uuid::Uuid;

use crate::{
    config::DbPool, formatters, models::team_member::{EditableTeamMember, NewTeamMember},
    repositories::TeamMemberRepository, utils::db::with_transaction,
};

pub struct TeamMemberService;

impl TeamMemberService {
    pub async fn create_team_member(pool: &DbPool, player_data: NewTeamMember) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamMemberRepository::create(tx, player_data).await })
        })
        .await
        {
            Ok(player) => {
                formatters::success_response(StatusCode::CREATED, player, "TEAM_MEMBER_ADDED")
            }
            Err(e) => {
                let error = e.to_string();
                let error_message = match error.as_str() {
                    err => err,
                };
                formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    error_message,
                    "TEAM PLAYER CREATION ERROR",
                )
            }
        }
    }

    pub async fn get_team_members(pool: &DbPool, team_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamMemberRepository::get_by_team(tx, team_id).await })
        })
        .await
        {
            Ok(members) => formatters::success_response(StatusCode::OK, members, "TEAM_MEMBERS_FETCHED"),
            Err(e) => {
                let error_message = e.to_string();
                formatters::error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    &error_message,
                    "TEAM MEMBERS FETCH ERROR",
                )
            }
        }
    }

    pub async fn get_team_member(pool: &DbPool, team_id: Uuid, player_id: Uuid) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamMemberRepository::get_by_id(tx, team_id, player_id).await })
        })
        .await
        {
            Ok(member) => formatters::success_response(StatusCode::OK, member, "TEAM_MEMBER_FETCHED"),
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Team member not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TEAM MEMBER FETCH ERROR",
                )
            }
        }
    }

    pub async fn update_team_member(
        pool: &DbPool,
        team_id: Uuid,
        player_id: Uuid,
        data: EditableTeamMember,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move {
                TeamMemberRepository::update(tx, team_id, player_id, data).await
            })
        })
        .await
        {
            Ok(member) => formatters::success_response(StatusCode::OK, member, "TEAM_MEMBER_UPDATED"),
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Team member not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TEAM MEMBER UPDATE ERROR",
                )
            }
        }
    }

    pub async fn delete_team_member(
        pool: &DbPool,
        team_id: Uuid,
        player_id: Uuid,
    ) -> HttpResponse {
        match with_transaction(pool, |tx| {
            Box::pin(async move { TeamMemberRepository::delete(tx, team_id, player_id).await })
        })
        .await
        {
            Ok(member) => formatters::success_response(StatusCode::OK, member, "TEAM_MEMBER_REMOVED"),
            Err(e) => {
                let error_message = if e.to_string().contains("no rows returned") {
                    "Team member not found"
                } else {
                    &e.to_string()
                };
                formatters::error_response(
                    StatusCode::NOT_FOUND,
                    error_message,
                    "TEAM MEMBER DELETE ERROR",
                )
            }
        }
    }
}
