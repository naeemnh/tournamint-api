use actix_web::{web, Responder};
use uuid::Uuid;

use crate::{
    config::DbPool,
    models::tournament_registration::{EditableTournamentRegistration, NewTournamentRegistration},
    services::TournamentRegistrationService,
};

pub struct TournamentRegistrationController;

impl TournamentRegistrationController {
    pub async fn create(
        pool: web::Data<DbPool>,
        data: web::Json<NewTournamentRegistration>,
    ) -> impl Responder {
        TournamentRegistrationService::create_registration(&pool, data.into_inner()).await
    }

    pub async fn get_by_id(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentRegistrationService::get_registration_by_id(&pool, id.into_inner()).await
    }

    pub async fn get_by_tournament_category(
        pool: web::Data<DbPool>,
        tournament_category_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentRegistrationService::get_registrations_by_tournament_category(
            &pool,
            tournament_category_id.into_inner(),
        )
        .await
    }

    pub async fn get_by_tournament(
        pool: web::Data<DbPool>,
        tournament_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentRegistrationService::get_registrations_by_tournament(
            &pool,
            tournament_id.into_inner(),
        )
        .await
    }

    pub async fn get_by_player(
        pool: web::Data<DbPool>,
        player_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentRegistrationService::get_registrations_by_player(&pool, player_id.into_inner())
            .await
    }

    pub async fn get_by_team(
        pool: web::Data<DbPool>,
        team_id: web::Path<Uuid>,
    ) -> impl Responder {
        TournamentRegistrationService::get_registrations_by_team(&pool, team_id.into_inner()).await
    }

    pub async fn update(
        pool: web::Data<DbPool>,
        id: web::Path<Uuid>,
        data: web::Json<EditableTournamentRegistration>,
    ) -> impl Responder {
        TournamentRegistrationService::update_registration(
            &pool,
            id.into_inner(),
            data.into_inner(),
        )
        .await
    }

    pub async fn delete(pool: web::Data<DbPool>, id: web::Path<Uuid>) -> impl Responder {
        TournamentRegistrationService::delete_registration(&pool, id.into_inner()).await
    }
}