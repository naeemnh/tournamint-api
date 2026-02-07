use actix_web::{web, HttpResponse};
use oauth2::{CsrfToken, Scope};

use crate::application::AuthServices;
use crate::infra::db::{PgTokenRepository, PgUserRepository};
use crate::shared::google;

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

    /// Handle Google OAuth callback — redirects to client with token and user as query params
    pub async fn google_callback(
        auth_services: web::Data<std::sync::Arc<AuthServices<PgUserRepository, PgTokenRepository>>>,
        query: web::Query<GoogleCallbackQuery>,
    ) -> HttpResponse {
        let client_redirect_url = std::env::var("CLIENT_REDIRECT_URL")
            .unwrap_or_else(|_| "http://localhost:3000/auth/callback".to_string());

        if query.code.is_empty() {
            let redirect = format!(
                "{}?error={}",
                client_redirect_url,
                urlencoding::encode("Missing authorization code")
            );
            return HttpResponse::Found()
                .append_header(("Location", redirect))
                .finish();
        }

        match auth_services.handle_google_login(&query.code).await {
            Ok(login_response) => {
                let user_json = serde_json::to_string(&login_response.user).unwrap_or_default();
                let redirect = format!(
                    "{}?token={}&user={}",
                    client_redirect_url,
                    urlencoding::encode(&login_response.jwt),
                    urlencoding::encode(&user_json),
                );
                HttpResponse::Found()
                    .append_header(("Location", redirect))
                    .finish()
            }
            Err(e) => {
                let redirect = format!(
                    "{}?error={}",
                    client_redirect_url,
                    urlencoding::encode(&e.to_string())
                );
                HttpResponse::Found()
                    .append_header(("Location", redirect))
                    .finish()
            }
        }
    }
}

#[derive(serde::Deserialize)]
pub struct GoogleCallbackQuery {
    pub code: String,
    #[serde(default)]
    pub state: String,
}
