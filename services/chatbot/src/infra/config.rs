use std::env;

pub struct Config {
    pub bot_token: String,
    #[allow(dead_code)]
    pub database_path: String,
    pub files_region: Option<String>,
    pub files_endpoint: Option<String>,
    pub files_bucket: Option<String>,
    pub files_key: Option<String>,
    pub files_secret: Option<String>,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bot_token: env::var("CHATBOT_TOKEN").expect("CHATBOT_TOKEN must be set"),
            database_path: env::var("CHATBOT_DATABASE").expect("CHATBOT_DATABASE must be set"),
            files_region: env::var("FILES_REGION").ok(),
            files_endpoint: env::var("FILES_ENDPOINT").ok(),
            files_bucket: env::var("FILES_BUCKET").ok(),
            files_key: env::var("FILES_KEY").ok(),
            files_secret: env::var("FILES_SECRET").ok(),
        }
    }
}
