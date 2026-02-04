use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};

pub fn get_google_oauth_client() -> BasicClient {
    let client_id =
        ClientId::new(std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));
    let client_secret = ClientSecret::new(
        std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
    );
    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
    let redirect_url = RedirectUrl::new(
        std::env::var("GOOGLE_REDIRECT_URL").expect("GOOGLE_REDIRECT_URL must be set"),
    )
    .unwrap();

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(redirect_url)
}
