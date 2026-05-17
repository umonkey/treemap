use crate::services::i18n::I18n;
use std::sync::Arc;
use teloxide::payloads::SetMyCommandsSetters;
use teloxide::prelude::*;
use teloxide::types::{BotCommand, ParseMode};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "report tree damage")]
    Report,
    #[command(description = "view the tree map")]
    Map,
}

pub async fn run(token: String, i18n: Arc<I18n>) {
    let bot = Bot::new(token);

    // Register commands in the Telegram menu button for each language
    for lang in ["en", "ru"] {
        let commands = vec![
            BotCommand::new("start", i18n.tr("menu-start-desc", lang, None)),
            BotCommand::new("report", i18n.tr("menu-report-desc", lang, None)),
            BotCommand::new("map", i18n.tr("menu-map-desc", lang, None)),
        ];

        let _ = bot
            .set_my_commands(commands)
            .language_code(lang)
            .await
            .map_err(|e| log::error!("Failed to set commands for {}: {:?}", lang, e));
    }

    log::info!("Chatbot is running...");

    teloxide::repl(bot, move |bot: Bot, msg: Message| {
        let i18n = i18n.clone();
        async move {
            // 1. Log the user info as requested
            let user_info = msg
                .from
                .as_ref()
                .map(|u| format!("{:?}", u))
                .unwrap_or_else(|| "Unknown".to_string());

            log::info!("Received message from: {}", user_info);

            // Get user language
            let lang = msg
                .from
                .as_ref()
                .and_then(|u| u.language_code.as_deref())
                .unwrap_or("en");

            // 2. Handle commands or echo text
            if let Some(text) = msg.text() {
                log::info!("Message text: {}", text);

                match Command::parse(text, "") {
                    Ok(Command::Start) => {
                        bot.send_message(msg.chat.id, i18n.tr("start-welcome", lang, None))
                            .parse_mode(ParseMode::Html)
                            .await?;
                    }
                    Ok(Command::Report) => {
                        bot.send_message(msg.chat.id, i18n.tr("report-intro", lang, None))
                            .parse_mode(ParseMode::Html)
                            .await?;
                    }
                    Ok(Command::Map) => {
                        bot.send_message(msg.chat.id, i18n.tr("map-link", lang, None))
                            .parse_mode(ParseMode::Html)
                            .await?;
                    }
                    Err(_) => {
                        // Not a command, just echo
                        bot.send_message(msg.chat.id, text).await?;
                    }
                }
            }

            Ok(())
        }
    })
    .await;
}
