use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::player::{CreatePlayer, EditablePlayer},
    services::player_service,
};

pub async fn index(pool: web::Data<DbPool>) -> impl Responder {
    player_service::get_all_players(&pool).await
}

pub async fn post(pool: web::Data<DbPool>, player_data: web::Json<CreatePlayer>) -> impl Responder {
    player_service::create_player(&pool, player_data.into_inner()).await
}

pub async fn show(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    player_service::get_player(&pool, id.into_inner()).await
}

pub async fn update(
    pool: web::Data<DbPool>,
    player_data: web::Json<EditablePlayer>,
    id: web::Path<Uuid>,
) -> impl Responder {
    player_service::update_player(&pool, id.into_inner(), player_data.into_inner()).await
}

pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    player_service::delete_player(&pool, id.into_inner()).await
}
