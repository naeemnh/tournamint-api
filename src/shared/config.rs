use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = env::var("APP_URL").unwrap_or_else(|_| "127.0.0.1".to_string());

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse::<u16>()
            .expect("APP_PORT must be a valid port number");

        Self { host, port }
    }

    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub struct EnvConfig {
    pub cloudinary_url: String,
    pub client_redirect_url: String,
    pub jwt_secret: String,
    pub database_url: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
}

impl EnvConfig {
    pub fn from_env() -> Self {
        Self {
            cloudinary_url: env::var("CLOUDINARY_URL").expect("CLOUDINARY_URL must be set"),
            client_redirect_url: env::var("CLIENT_REDIRECT_URL")
                .expect("CLIENT_REDIRECT_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            google_client_id: env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET")
                .expect("GOOGLE_CLIENT_SECRET must be set"),
            google_redirect_url: env::var("GOOGLE_REDIRECT_URL")
                .expect("GOOGLE_REDIRECT_URL must be set"),
        }
    }
}
