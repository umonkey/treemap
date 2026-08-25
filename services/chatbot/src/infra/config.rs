use crate::infra::secrets::Secrets;
use std::env;

pub struct Config {
    pub bot_token: String,
    pub database_path: String,
    pub files_region: String,
    pub files_endpoint: String,
    pub files_bucket: String,
    pub files_key: String,
    pub files_secret: String,
    pub files_base_url: String,
    pub website_url: String,
    pub report_recipients: Vec<i64>,
}

impl Config {
    pub fn from_env() -> Self {
        let secrets_path = env::var("SECRETS_PATH").unwrap_or_else(|_| "/run/secrets".to_string());
        let secrets = Secrets::new(&secrets_path).expect("Failed to load secrets");

        let report_recipients = secrets
            .report_recipients
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .filter_map(|s| match s.parse::<i64>() {
                Ok(id) => Some(id),
                Err(e) => {
                    log::warn!("Invalid report recipient ID '{}': {}", s, e);
                    None
                }
            })
            .collect();

        Self {
            bot_token: secrets.bot_token,
            database_path: env::var("CHATBOT_DATABASE").expect("CHATBOT_DATABASE must be set"),
            files_region: env::var("FILES_REGION").expect("FILES_REGION must be set"),
            files_endpoint: env::var("FILES_ENDPOINT").expect("FILES_ENDPOINT must be set"),
            files_bucket: env::var("FILES_BUCKET").expect("FILES_BUCKET must be set"),
            files_key: secrets.files_key,
            files_secret: secrets.files_secret,
            files_base_url: env::var("FILES_BASE_URL").expect("FILES_BASE_URL must be set"),
            website_url: env::var("WEBSITE_URL")
                .unwrap_or_else(|_| "http://localhost:5173".to_string()),
            report_recipients,
        }
    }
}
