use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tournament_category::{EditableTournamentCategory, NewTournamentCategory},
    services::TournamentCategoryService,
};

pub struct TournamentCategoryController;

impl TournamentCategoryController {
    pub async fn create(
        pool: web::Data<DbPool>,
        data: web::Json<NewTournamentCategory>,
    ) -> impl Responder {
        TournamentCategoryService::create_category(&pool, data.into_inner()).await
    }

    pub async fn get_by_tournament(
        pool: web::Data<DbPool>,
        tournament_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentCategoryService::get_categories_by_tournament(&pool, tournament_id.into_inner())
            .await
    }

    pub async fn get_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentCategoryService::get_category_by_id(&pool, id.into_inner()).await
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        data: web::Json<EditableTournamentCategory>,
    ) -> impl Responder {
        TournamentCategoryService::update_category(&pool, id.into_inner(), data.into_inner()).await
    }

    pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentCategoryService::delete_category(&pool, id.into_inner()).await
    }
}