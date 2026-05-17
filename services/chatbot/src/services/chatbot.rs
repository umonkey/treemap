use crate::domains::photo::PhotoRepository;
use crate::domains::report::{Report, ReportRepository};
use crate::domains::tree::TreeRepository;
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
    reports: Arc<ReportRepository>,
    photos: Arc<PhotoRepository>,
    trees: Arc<TreeRepository>,
}

impl Chatbot {
    pub fn new(
        token: String,
        i18n: Arc<I18n>,
        reports: Arc<ReportRepository>,
        photos: Arc<PhotoRepository>,
        trees: Arc<TreeRepository>,
    ) -> Self {
        Self {
            bot: Bot::new(token),
            i18n,
            reports,
            photos,
            trees,
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
        let user_id_str = user
            .map(|u| u.id.to_string())
            .unwrap_or_else(|| "?".to_string());
        let user_id = user.map(|u| u.id.0 as i64).unwrap_or(0);
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
            user_id_str,
            user_alias,
            raw_lang,
            text_content
        );

        // 1. Handle commands
        if let Some(text) = msg.text() {
            if let Ok(cmd) = Command::parse(text, "") {
                return self.handle_command(&msg, cmd, lang).await;
            }
        }

        let mut report_id = None;
        let mut should_respond = false;

        // 2. Process photos
        if let Some(photos) = msg.photo() {
            if let Ok(report) = self.start_report(&msg, false, lang).await {
                report_id = Some(report.id);
                if let Some(photo) = photos.last() {
                    if let Ok(is_first) = self.photos.add_to_report(report.id, &photo.file.id).await
                    {
                        if is_first {
                            should_respond = true;
                        }
                    }
                }
            }
        }

        // 3. Process location
        if let Some(location) = msg.location() {
            if let Ok(report) = self.start_report(&msg, false, lang).await {
                report_id = Some(report.id);
                let _ = self
                    .reports
                    .update_location(report.id, location.latitude, location.longitude)
                    .await;
                should_respond = true;
            }
        }

        // 4. Process text (description)
        if !text_content.is_empty() {
            if report_id.is_none() {
                report_id = self
                    .reports
                    .get_active_id_by_user_id(user_id)
                    .await
                    .unwrap_or(None);
            }

            if let Some(rid) = report_id {
                let _ = self.reports.update_description(rid, text_content).await;
                should_respond = true;
            }
        }

        // 5. Send response based on report status
        if should_respond {
            if let Some(rid) = report_id {
                self.send_next_step(msg.chat.id, rid, lang).await?;
            }
        }

        Ok(())
    }

    async fn send_next_step(
        &self,
        chat_id: ChatId,
        report_id: i64,
        lang: &str,
    ) -> ResponseResult<()> {
        let report = match self.reports.get_by_id(report_id).await {
            Ok(Some(r)) => r,
            _ => return Ok(()),
        };

        let photo_count = self.photos.count_by_report_id(report_id).await.unwrap_or(0);

        let text = if photo_count == 0 {
            self.i18n.tr("report-intro", lang, None)
        } else if report.lat.is_none() {
            self.i18n.tr("report-photo-received", lang, None)
        } else if report.description.is_none() {
            self.i18n.tr("report-location-received", lang, None)
        } else {
            self.i18n.tr("report-completed", lang, None)
        };

        self.bot
            .send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .await?;

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
    ) -> anyhow::Result<Report> {
        let user = msg.from.as_ref();
        let user_id = user.map(|u| u.id.0 as i64).unwrap_or(0);
        let chat_id = msg.chat.id.0;
        let username = user.and_then(|u| u.username.clone());
        let location = msg.location();
        let lat = location.map(|l| l.latitude);
        let lon = location.map(|l| l.longitude);

        self.reports
            .create(
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
        let count = self.trees.count().await.unwrap_or(0);
        args.set("count", count);

        self.bot
            .send_message(msg.chat.id, self.i18n.tr("map-link", lang, Some(&args)))
            .parse_mode(ParseMode::Html)
            .reply_markup(keyboard)
            .await?;

        Ok(())
    }
}

pub async fn run(
    token: String,
    i18n: Arc<I18n>,
    reports: Arc<ReportRepository>,
    photos: Arc<PhotoRepository>,
    trees: Arc<TreeRepository>,
) {
    let chatbot = Arc::new(Chatbot::new(token, i18n, reports, photos, trees));
    chatbot.run().await;
}
