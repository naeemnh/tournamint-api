use crate::{services::AuthService, utils::google};
use actix_web::{web, HttpResponse, Responder};
use oauth2::{CsrfToken, Scope};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct GoogleCallbackQuery {
    code: String,
}

pub struct AuthController;

impl AuthController {
    pub async fn start_google_login() -> impl Responder {
        let client = google::get_google_oauth_client();

        // Generate the authorization URL
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("access_type", "offline")
            .add_extra_param("prompt", "consent")
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        // Redirect to Google's authorization page
        HttpResponse::Found()
            .append_header(("Location", auth_url.to_string()))
            .finish()
    }

    pub async fn google_callback(
        pool: web::Data<PgPool>,
        code: web::Query<GoogleCallbackQuery>,
    ) -> impl Responder {
        AuthService::handle_google_login(&pool, &code.code).await
    }
}
