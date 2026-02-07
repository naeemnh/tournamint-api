use chrono::Utc;
use oauth2::TokenResponse;
use std::sync::Arc;

use crate::domain::user::{LoginResponse, NewUser, TokenRepository, UserRepository, UserToken};
use crate::shared::{google, jwt, AppError};

/// Auth services (Google OAuth login, etc.)
pub struct AuthServices<U, T>
where
    U: UserRepository,
    T: TokenRepository,
{
    user_repo: Arc<U>,
    token_repo: Arc<T>,
}

impl<U, T> AuthServices<U, T>
where
    U: UserRepository,
    T: TokenRepository,
{
    pub fn new(user_repo: Arc<U>, token_repo: Arc<T>) -> Self {
        Self {
            user_repo,
            token_repo,
        }
    }

    /// Exchange Google auth code for tokens, fetch user info, find-or-create user, upsert refresh token, return JWT and user.
    pub async fn handle_google_login(&self, code: &str) -> Result<LoginResponse, AppError> {
        let client = google::get_google_oauth_client();

        let token_response = client
            .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
            .request_async(oauth2::reqwest::async_http_client)
            .await
            .map_err(|e| AppError::BadRequest(format!("Token exchange failed: {}", e)))?;

        let access_token = token_response.access_token().secret();
        let refresh_token = token_response
            .refresh_token()
            .map(|t| t.secret().to_string());
        let expires_in = token_response
            .expires_in()
            .map(|d| d.as_secs() as i64)
            .unwrap_or(3600);

        let http_client = reqwest::Client::new();
        let user_info: serde_json::Value = http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to fetch user info: {}", e)))?
            .json()
            .await
            .map_err(|e| AppError::BadRequest(format!("Failed to parse user info: {}", e)))?;

        let google_id = user_info["id"]
            .as_str()
            .ok_or_else(|| AppError::BadRequest("Missing user id from Google".to_string()))?
            .to_string();
        let email = user_info["email"].as_str().unwrap_or("").to_string();
        let name = user_info["name"].as_str().map(|s| s.to_string());

        let new_user = NewUser {
            google_id,
            email,
            name,
        };

        let user = match self
            .user_repo
            .find_by_google_id(&new_user.google_id)
            .await?
        {
            Some(u) => u,
            None => self.user_repo.create(new_user).await?,
        };

        if let Some(refresh) = refresh_token {
            self.token_repo
                .upsert_refresh_token(UserToken {
                    user_id: user.id,
                    refresh_token: refresh,
                    expires_at: Utc::now() + chrono::Duration::seconds(expires_in),
                })
                .await?;
        }

        let jwt = jwt::generate_jwt(user.id, &user.email)
            .map_err(|e| AppError::BadRequest(format!("JWT generation failed: {}", e)))?;

        Ok(LoginResponse { user, jwt })
    }
}
