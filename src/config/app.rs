use std::env;

use dotenv::dotenv;

pub struct AppConfig {
    #[allow(dead_code)]
    pub app_name: String,
    pub app_host: String,
    pub app_port: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        Self {
            app_name: env::var("APP_NAME").expect("App name is missing form environment variables"),
            app_host: env::var("APP_HOST")
                .expect("App host is not defined in the environment variables"),
            app_port: env::var("APP_PORT")
                .expect("App port is not defined in the environment variables"),
        }
    }

    pub fn get_app_url() -> String {
        let app_config = Self::from_env();
        format!("{}:{}", app_config.app_host, app_config.app_port)
    }
}
