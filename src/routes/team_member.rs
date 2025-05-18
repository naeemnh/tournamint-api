use actix_web::web;

use crate::controllers::team_member_controller;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/team_members").route("", web::post().to(team_member_controller::post)),
    );
}
