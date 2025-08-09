use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tournament_standings::StandingsUpdateRequest,
    services::TournamentStandingsService,
};

pub struct TournamentStandingsController;

impl TournamentStandingsController {
    pub async fn get_tournament_standings(
        pool: web::Data<DbPool>,
        tournament_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentStandingsService::get_tournament_standings(&pool, tournament_id.into_inner()).await
    }

    pub async fn get_category_standings(
        pool: web::Data<DbPool>,
        category_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentStandingsService::get_category_standings(&pool, category_id.into_inner()).await
    }

    pub async fn update_standings(
        pool: web::Data<DbPool>,
        tournament_id: web::Path<Uuid>,
        data: web::Json<StandingsUpdateRequest>,
    ) -> impl Responder {
        TournamentStandingsService::update_standings(
            &pool,
            tournament_id.into_inner(),
            data.into_inner(),
        )
        .await
    }
}