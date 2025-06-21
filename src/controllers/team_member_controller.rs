use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::team_member::{EditableTeamMember, NewTeamMember},
    services::TeamMemberService,
};

pub struct TeamMemberController;

impl TeamMemberController {
    pub async fn post(
        pool: web::Data<DbPool>,
        player_data: web::Json<NewTeamMember>,
    ) -> impl Responder {
        TeamMemberService::create_team_member(&pool, player_data.into_inner()).await
    }

    pub async fn get_by_team(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TeamMemberService::get_team_members(&pool, id.into_inner()).await
    }

    pub async fn get_by_id(
        pool: web::Data<DbPool>,
        path: web::Path<(Uuid, Uuid)>,
    ) -> impl Responder {
        let (team_id, player_id) = path.into_inner();
        TeamMemberService::get_team_member(&pool, team_id, player_id).await
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        path: web::Path<(Uuid, Uuid)>,
        data: web::Json<EditableTeamMember>,
    ) -> impl Responder {
        let (team_id, player_id) = path.into_inner();
        TeamMemberService::update_team_member(&pool, team_id, player_id, data.into_inner()).await
    }

    pub async fn delete(pool: web::Data<DbPool>, path: web::Path<(Uuid, Uuid)>) -> impl Responder {
        let (team_id, player_id) = path.into_inner();
        TeamMemberService::delete_team_member(&pool, team_id, player_id).await
    }
}
