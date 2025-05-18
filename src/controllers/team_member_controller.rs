use actix_web::{web, Responder};

use crate::{config::DbPool, models::team_member::NewTeamMember, services::team_member_service};

pub async fn post(
    pool: web::Data<DbPool>,
    player_data: web::Json<NewTeamMember>,
) -> impl Responder {
    team_member_service::create_team_member(&pool, player_data.into_inner()).await
}
