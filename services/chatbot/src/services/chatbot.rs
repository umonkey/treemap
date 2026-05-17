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
            // 1. Log the user info and content
            let user = msg.from.as_ref();
            let user_name = user
                .map(|u| u.full_name())
                .unwrap_or_else(|| "Unknown".to_string());
            let user_id = user
                .map(|u| u.id.to_string())
                .unwrap_or_else(|| "?".to_string());
            let text_content = msg.text().or(msg.caption()).unwrap_or("");

            log::info!(
                "Received message from {} ({}): {}",
                user_name,
                user_id,
                text_content
            );

            // Log location if present
            if let Some(loc) = msg.location() {
                log::info!(
                    "Location received from {} ({}): lat={}, lon={}",
                    user_name,
                    user_id,
                    loc.latitude,
                    loc.longitude
                );
            }

            // Get user language
            let lang = user
                .and_then(|u| u.language_code.as_deref())
                .unwrap_or("en");

            // 2. Handle commands or echo text
            if let Some(text) = msg.text() {
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
