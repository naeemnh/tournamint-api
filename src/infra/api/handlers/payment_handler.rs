use actix_web::{web, HttpResponse, ResponseError};

use crate::application::PaymentUseCases;
use crate::domain::payment::NewPayment;
use crate::infra::db::PgPaymentRepository;
use crate::shared::ApiResponse;

type PaymentUseCasesData = std::sync::Arc<PaymentUseCases<PgPaymentRepository>>;

pub struct PaymentHandler;

impl PaymentHandler {
    pub async fn process(
        use_cases: web::Data<PaymentUseCasesData>,
        body: web::Json<NewPayment>,
    ) -> HttpResponse {
        match use_cases.process_payment(body.into_inner()).await {
            Ok(payment) => ApiResponse::created("Created", payment),
            Err(e) => e.error_response(),
        }
    }
}
