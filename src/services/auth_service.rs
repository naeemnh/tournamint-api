use actix_web::HttpResponse;
use chrono::Utc;
use oauth2::{http::StatusCode, TokenResponse};

use crate::{
    config::DbPool,
    formatters,
    models::{auth::LoginResponse, token::UserToken, user::NewUser},
    repositories::{token_repository, user_repository},
    utils::{db::with_transaction, google, jwt::generate_jwt},
};

pub async fn handle_google_login(pool: &DbPool, code: &str) -> HttpResponse {
    let client = google::get_google_oauth_client();

    // Exchange code for token
    let token_response = match client
        .exchange_code(oauth2::AuthorizationCode::new(code.to_string()))
        .request_async(oauth2::reqwest::async_http_client)
        .await
    {
        Ok(token) => token,
        Err(e) => {
            return formatters::error_response(
                StatusCode::BAD_REQUEST,
                &format!("Token exchange failed: {}", e),
                "AUTH_ERROR",
            );
        }
    };

    let access_token = token_response.access_token().secret();
    let refresh_token = token_response
        .refresh_token()
        .map(|t| t.secret().to_string());
    let expires_in = token_response.expires_in().unwrap().as_secs() as i64;

    // Get user info from Google
    let client = reqwest::Client::new();
    let user_info: serde_json::Value = match client
        .get("https://www.googleapis.com/oauth2/v2/userinfo")
        .bearer_auth(access_token)
        .send()
        .await
    {
        Ok(response) => match response.json().await {
            Ok(info) => info,
            Err(e) => {
                return formatters::error_response(
                    StatusCode::BAD_REQUEST,
                    &format!("Failed to parse user info: {}", e),
                    "AUTH_ERROR",
                );
            }
        },
        Err(e) => {
            return formatters::error_response(
                StatusCode::BAD_REQUEST,
                &format!("Failed to fetch user info: {}", e),
                "AUTH_ERROR",
            );
        }
    };

    let new_user = NewUser {
        google_id: user_info["id"].to_string(),
        email: user_info["email"].to_string(),
        name: Some(user_info["name"].to_string()),
    };

    // Find or create user
    match with_transaction(pool, |tx| {
        Box::pin(async move {
            let user = if let Some(user) =
                user_repository::find_by_google_id(tx, &new_user.google_id).await?
            {
                user
            } else {
                user_repository::create(tx, new_user).await?
            };

            if let Some(refresh_token) = refresh_token {
                token_repository::upsert_refresh_token(
                    tx,
                    UserToken {
                        user_id: user.id,
                        refresh_token,
                        expires_at: Utc::now() + chrono::Duration::seconds(expires_in),
                    },
                )
                .await?;
            }

            let jwt = generate_jwt(&user)
                .map_err(|e| sqlx::Error::Configuration(e.to_string().into()))?;

            Ok(LoginResponse { user, jwt })
        })
    })
    .await
    {
        Ok(result) => formatters::success_response(StatusCode::OK, result, "LOGGED IN"),
        Err(e) => {
            let error = e.to_string();
            let error_message = match error.as_str() {
                err => err,
            };
            formatters::error_response(
                StatusCode::BAD_REQUEST,
                error_message,
                "PLAYER_CREATION_ERROR",
            )
        }
    }
}
