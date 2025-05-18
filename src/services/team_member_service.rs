use actix_web::HttpResponse;
use oauth2::http::StatusCode;

use crate::{
    config::DbPool, formatters, models::team_member::NewTeamMember,
    repositories::team_member_repository, utils::db::with_transaction,
};

pub async fn create_team_member(pool: &DbPool, player_data: NewTeamMember) -> HttpResponse {
    match with_transaction(pool, |tx| {
        Box::pin(async move { team_member_repository::create(tx, player_data).await })
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
