use std::env;

pub struct DbConfig {
    database_url: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL mus be set in .env file or environment variables"),
        }
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}
