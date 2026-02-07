use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

use crate::shared::EnvConfig;

pub fn get_google_oauth_client() -> BasicClient {
    let client_id = ClientId::new(EnvConfig::from_env().google_client_id);
    let client_secret = ClientSecret::new(EnvConfig::from_env().google_client_secret);
    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
    let redirect_url = RedirectUrl::new(EnvConfig::from_env().google_redirect_url).unwrap();

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url)
}
