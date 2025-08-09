use actix_web::web;

use crate::controllers::payment_controller::PaymentController;
use crate::middlewares::auth::auth_middleware;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .wrap(auth_middleware())
        .route("/process", web::post().to(PaymentController::process_payment))
        .route("/{id}", web::get().to(PaymentController::get_payment))
        .route("/user/{user_id}", web::get().to(PaymentController::get_user_payments))
        .route("/tournament/{tournament_id}", web::get().to(PaymentController::get_tournament_payments))
        .route("/{id}/refund", web::put().to(PaymentController::refund_payment))
        .route("/{id}/status", web::get().to(PaymentController::get_payment_status))
        .route("/{id}/status", web::put().to(PaymentController::update_payment_status))
        .route("/summary/tournament/{tournament_id}", web::get().to(PaymentController::get_tournament_payment_summary))
        .route("/summary/user", web::get().to(PaymentController::get_user_payment_summary))
    );
}