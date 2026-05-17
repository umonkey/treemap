use crate::infra::database::DatabaseClient;
use crate::services::i18n::I18n;
use fluent::FluentArgs;
use std::sync::Arc;
use teloxide::payloads::SetMyCommandsSetters;
use teloxide::prelude::*;
use teloxide::types::{BotCommand, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode};
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

pub struct Chatbot {
    bot: Bot,
    i18n: Arc<I18n>,
    db: Arc<DatabaseClient>,
}

impl Chatbot {
    pub fn new(token: String, i18n: Arc<I18n>, db: Arc<DatabaseClient>) -> Self {
        Self {
            bot: Bot::new(token),
            i18n,
            db,
        }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.register_commands().await {
            log::error!("Failed to register commands: {:?}", e);
        }

        log::info!("Chatbot is running...");

        let bot = self.bot.clone();
        let chatbot = Arc::clone(&self);

        teloxide::repl(bot, move |_bot: Bot, msg: Message| {
            let chatbot = Arc::clone(&chatbot);
            async move { chatbot.handle_message(msg).await }
        })
        .await;
    }

    async fn register_commands(&self) -> ResponseResult<()> {
        // 1. Set default commands (English)
        let default_commands = vec![
            BotCommand::new("start", self.i18n.tr("menu-start-desc", "en", None)),
            BotCommand::new("report", self.i18n.tr("menu-report-desc", "en", None)),
            BotCommand::new("map", self.i18n.tr("menu-map-desc", "en", None)),
        ];

        let _ = self.bot.set_my_commands(default_commands).await;

        // 2. Set localized commands
        for lang in ["en", "ru"] {
            let commands = vec![
                BotCommand::new("start", self.i18n.tr("menu-start-desc", lang, None)),
                BotCommand::new("report", self.i18n.tr("menu-report-desc", lang, None)),
                BotCommand::new("map", self.i18n.tr("menu-map-desc", lang, None)),
            ];

            let _ = self.bot.set_my_commands(commands).language_code(lang).await;
        }

        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> ResponseResult<()> {
        let user = msg.from.as_ref();
        let user_name = user
            .map(|u| u.full_name())
            .unwrap_or_else(|| "Unknown".to_string());
        let user_id = user
            .map(|u| u.id.to_string())
            .unwrap_or_else(|| "?".to_string());
        let user_alias = user
            .and_then(|u| u.username.as_ref())
            .map(|username| format!(" (t.me/{})", username))
            .unwrap_or_default();
        let text_content = msg.text().or(msg.caption()).unwrap_or("");
        let raw_lang = user
            .and_then(|u| u.language_code.as_deref())
            .unwrap_or("en");
        let lang = &raw_lang[..2.min(raw_lang.len())];

        log::info!(
            "Received message from {} ({}{}): language={}, text={}",
            user_name,
            user_id,
            user_alias,
            raw_lang,
            text_content
        );

        if let Some(text) = msg.text() {
            match Command::parse(text, "") {
                Ok(cmd) => self.handle_command(&msg, cmd, lang).await?,
                Err(_) => self.handle_text(&msg, text).await?,
            };
        } else if msg.photo().is_some() || msg.location().is_some() {
            let _ = self.start_report(&msg, false, lang).await;

            if let Some(caption) = msg.caption() {
                self.handle_text(&msg, caption).await?;
            }
        }

        Ok(())
    }

    async fn handle_command(&self, msg: &Message, cmd: Command, lang: &str) -> ResponseResult<()> {
        match cmd {
            Command::Start => self.handle_start(msg, lang).await,
            Command::Report => self.handle_report(msg, lang).await,
            Command::Map => self.handle_map(msg, lang).await,
        }
    }

    async fn start_report(
        &self,
        msg: &Message,
        force_new: bool,
        lang: &str,
    ) -> anyhow::Result<i64> {
        let user = msg.from.as_ref();
        let user_id = user.map(|u| u.id.0 as i64).unwrap_or(0);
        let chat_id = msg.chat.id.0;
        let username = user.and_then(|u| u.username.clone());
        let location = msg.location();
        let lat = location.map(|l| l.latitude);
        let lon = location.map(|l| l.longitude);

        self.db
            .create_report(
                user_id,
                chat_id,
                Some(msg.id.0),
                username,
                Some(lang.to_string()),
                lat,
                lon,
                force_new,
            )
            .await
    }

    async fn handle_start(&self, msg: &Message, lang: &str) -> ResponseResult<()> {
        self.bot
            .send_message(msg.chat.id, self.i18n.tr("start-welcome", lang, None))
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    async fn handle_report(&self, msg: &Message, lang: &str) -> ResponseResult<()> {
        let _ = self.start_report(msg, true, lang).await;

        self.bot
            .send_message(msg.chat.id, self.i18n.tr("report-intro", lang, None))
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    async fn handle_map(&self, msg: &Message, lang: &str) -> ResponseResult<()> {
        let url = "https://yerevan.treemaps.app/".parse().unwrap();
        let button_label = format!("🌐 {}", self.i18n.tr("map-button-label", lang, None));
        let keyboard =
            InlineKeyboardMarkup::new(vec![vec![InlineKeyboardButton::url(button_label, url)]]);

        let mut args = FluentArgs::new();
        let count = self.db.count_trees().await.unwrap_or(0);
        args.set("count", count);

        self.bot
            .send_message(msg.chat.id, self.i18n.tr("map-link", lang, Some(&args)))
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }

    async fn handle_text(&self, msg: &Message, text: &str) -> ResponseResult<()> {
        self.bot.send_message(msg.chat.id, text).await?;
        Ok(())
    }
}

pub async fn run(token: String, i18n: Arc<I18n>, db: Arc<DatabaseClient>) {
    let chatbot = Arc::new(Chatbot::new(token, i18n, db));
    chatbot.run().await;
}
