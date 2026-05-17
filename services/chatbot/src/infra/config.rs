use std::env;

pub struct Config {
    pub bot_token: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            bot_token: env::var("CHATBOT_TOKEN").expect("CHATBOT_TOKEN must be set"),
        }
    }
}
