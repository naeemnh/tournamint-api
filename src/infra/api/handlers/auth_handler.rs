use actix_web::{web, HttpResponse, ResponseError};
use oauth2::{CsrfToken, Scope};

use crate::application::AuthUseCases;
use crate::infra::db::{PgTokenRepository, PgUserRepository};
use crate::shared::{google, ApiResponse};

/// Auth handler for Google OAuth
pub struct AuthHandler;

impl AuthHandler {
    /// Start Google OAuth flow - redirects to Google
    pub async fn start_google_login() -> HttpResponse {
        let client = google::get_google_oauth_client();

        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .add_extra_param("access_type", "offline")
            .add_extra_param("prompt", "consent")
            .add_scope(Scope::new("email".to_string()))
            .add_scope(Scope::new("profile".to_string()))
            .url();

        HttpResponse::Found()
            .append_header(("Location", auth_url.to_string()))
            .finish()
    }

    /// Handle Google OAuth callback
    pub async fn google_callback(
        auth_use_cases: web::Data<
            std::sync::Arc<AuthUseCases<PgUserRepository, PgTokenRepository>>,
        >,
        query: web::Query<GoogleCallbackQuery>,
    ) -> HttpResponse {
        if query.code.is_empty() {
            return ApiResponse::bad_request("Missing authorization code");
        }

        match auth_use_cases.handle_google_login(&query.code).await {
            Ok(login_response) => ApiResponse::success("Logged in", Some(login_response)),
            Err(e) => e.error_response(),
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
    #[serde(default)]
    pub state: String,
}
