use std::env;

pub struct Config {
    pub bot_token: String,
    #[allow(dead_code)]
    pub database_path: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bot_token: env::var("CHATBOT_TOKEN").expect("CHATBOT_TOKEN must be set"),
            database_path: env::var("CHATBOT_DATABASE").expect("CHATBOT_DATABASE must be set"),
        }
    }
}
