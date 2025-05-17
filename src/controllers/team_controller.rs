use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::team::{EditableTeam, NewTeam},
    services::team_service,
};

pub async fn index(pool: web::Data<DbPool>) -> impl Responder {
    team_service::get_all_teams(&pool).await
}

pub async fn post(pool: web::Data<DbPool>, team_data: web::Json<NewTeam>) -> impl Responder {
    team_service::create_team(&pool, team_data.into_inner()).await
}

pub async fn show(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    team_service::get_team(&pool, id.into_inner()).await
}

pub async fn update(
    pool: web::Data<DbPool>,
    team_data: web::Json<EditableTeam>,
    id: web::Path<Uuid>,
) -> impl Responder {
    team_service::update_team(&pool, id.into_inner(), team_data.into_inner()).await
}

pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    team_service::delete_team(&pool, id.into_inner()).await
}
