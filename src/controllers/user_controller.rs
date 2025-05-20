use actix_web::{web, Responder};
use uuid::Uuid;

use crate::config::db::DbPool;
use crate::models::user::{EditableUser, NewUser};
use crate::services::user_service;

pub async fn index(pool: web::Data<DbPool>) -> impl Responder {
    user_service::get_all_users(&pool).await
}

pub async fn post(pool: web::Data<DbPool>, user_data: web::Json<NewUser>) -> impl Responder {
    user_service::create_user(&pool, user_data.into_inner()).await
}

pub async fn show(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    user_service::get_user(&pool, id.into_inner()).await
}

pub async fn update(
    pool: web::Data<DbPool>,
    user_data: web::Json<EditableUser>,
    id: web::Path<Uuid>,
) -> impl Responder {
    user_service::update_user(&pool, id.into_inner(), user_data.into_inner()).await
}

pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
    user_service::delete_user(&pool, id.into_inner()).await
}
