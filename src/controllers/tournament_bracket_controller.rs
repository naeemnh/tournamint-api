use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tournament_bracket::GenerateBracketRequest,
    services::TournamentBracketService,
};

pub struct TournamentBracketController;

impl TournamentBracketController {
    pub async fn get_tournament_bracket(
        pool: web::Data<DbPool>,
        tournament_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentBracketService::get_tournament_bracket(&pool, tournament_id.into_inner()).await
    }

    pub async fn get_category_bracket(
        pool: web::Data<DbPool>,
        category_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentBracketService::get_category_bracket(&pool, category_id.into_inner()).await
    }

    pub async fn generate_bracket(
        pool: web::Data<DbPool>,
        tournament_id: web::Path<Uuid>,
        data: web::Json<GenerateBracketRequest>,
    ) -> impl Responder {
        TournamentBracketService::generate_bracket(
            &pool,
            tournament_id.into_inner(),
            data.into_inner(),
        )
        .await
    }
}