use crate::domains::alert::{Alert, AlertRepository};
use crate::domains::alert_photo::AlertPhotoRepository;
use crate::domains::tree::TreeRepository;
use crate::infra::s3::S3FileStorage;
use crate::services::i18n::I18n;
use crate::utils::id::get_unique_id;
use fluent::FluentArgs;
use std::sync::Arc;
use teloxide::net::Download;
use teloxide::payloads::SetMyCommandsSetters;
use teloxide::prelude::*;
use teloxide::types::{
    BotCommand, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode, ReactionType,
};
use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "start the bot")]
    Start,
    #[command(description = "alert tree damage")]
    Alert,
    #[command(description = "view the tree map")]
    Map,
}

pub struct Chatbot {
    bot: Bot,
    bot_id: UserId,
    bot_username: String,
    i18n: Arc<I18n>,
    alerts: Arc<AlertRepository>,
    photos: Arc<AlertPhotoRepository>,
    trees: Arc<TreeRepository>,
    storage: Arc<S3FileStorage>,
}

impl Chatbot {
    pub fn new(
        token: String,
        bot_id: UserId,
        bot_username: String,
        i18n: Arc<I18n>,
        alerts: Arc<AlertRepository>,
        photos: Arc<AlertPhotoRepository>,
        trees: Arc<TreeRepository>,
        storage: Arc<S3FileStorage>,
    ) -> Self {
        Self {
            bot: Bot::new(token),
            bot_id,
            bot_username,
            i18n,
            alerts,
            photos,
            trees,
            storage,
        }
    }

    pub async fn run(self: Arc<Self>) {
        if let Err(e) = self.register_commands().await {
            log::error!("Failed to register commands: {:?}", e);
        }

        log::info!(
            "Chatbot is running as @{} ({})...",
            self.bot_username,
            self.bot_id
        );

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
            BotCommand::new("alert", self.i18n.tr("menu-alert-desc", "en", None)),
            BotCommand::new("map", self.i18n.tr("menu-map-desc", "en", None)),
        ];

        let _ = self.bot.set_my_commands(default_commands).await;

        // 2. Set localized commands
        for lang in ["en", "ru"] {
            let commands = vec![
                BotCommand::new("start", self.i18n.tr("menu-start-desc", lang, None)),
                BotCommand::new("alert", self.i18n.tr("menu-alert-desc", lang, None)),
                BotCommand::new("map", self.i18n.tr("menu-map-desc", lang, None)),
            ];

            let _ = self.bot.set_my_commands(commands).language_code(lang).await;
        }

        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> ResponseResult<()> {
        if let Some(new_members) = msg.new_chat_members() {
            if new_members.iter().any(|u| u.id == self.bot_id) {
                log::info!(
                    "Bot added to group chat '{}' ({})",
                    msg.chat.title().unwrap_or("?"),
                    msg.chat.id
                );
            }
        }

        if let Some(left_member) = msg.left_chat_member() {
            if left_member.id == self.bot_id {
                log::info!(
                    "Bot removed from group chat '{}' ({})",
                    msg.chat.title().unwrap_or("?"),
                    msg.chat.id
                );
            }
        }

        if !msg.chat.is_private() {
            let user_name = msg
                .from
                .as_ref()
                .map(|u| u.full_name())
                .unwrap_or_else(|| "Unknown".to_string());

            log::debug!(
                "Ignoring message to group chat {} from {}",
                msg.chat.id,
                user_name
            );

            return Ok(());
        }

        if let Err(e) = self.handle_message_impl(msg.clone()).await {
            log::error!("Error handling message: {:?}", e);

            let raw_lang = msg
                .from
                .as_ref()
                .and_then(|u| u.language_code.as_deref())
                .unwrap_or("en");
            let lang = &raw_lang[..2.min(raw_lang.len())];

            let error_text = self.i18n.tr("generic-error", lang, None);
            let _ = self.bot.send_message(msg.chat.id, error_text).await;
        }

        Ok(())
    }

    async fn handle_message_impl(&self, msg: Message) -> anyhow::Result<()> {
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

        let mut alert_id = None;
        let mut should_respond = false;

        // 2. Process photos
        if let Some(photos) = msg.photo() {
            let alert = self.start_alert(&msg, false, lang).await?;
            alert_id = Some(alert.id);

            if let Some(photo) = photos.last() {
                match self.process_photo(&msg, &alert, photo).await {
                    Ok(is_first) => {
                        if is_first {
                            should_respond = true;
                        }
                    }
                    Err(e) => {
                        log::error!("Error processing photo: {:?}", e);
                        let error_text = self.i18n.tr("generic-error", lang, None);
                        let _ = self.bot.send_message(msg.chat.id, error_text).await;
                    }
                }
            }
        } else if msg.video().is_some() || msg.document().is_some() {
            let error_text = self.i18n.tr("alert-unsupported-file-type", lang, None);
            let _ = self.bot.send_message(msg.chat.id, error_text).await;
            return Ok(());
        }

        // 3. Process location
        if let Some(location) = msg.location() {
            let alert = self.start_alert(&msg, false, lang).await?;
            alert_id = Some(alert.id);
            self.alerts
                .update_location(alert.id, location.latitude, location.longitude)
                .await?;
            should_respond = true;
        }

        // 4. Process text (description)
        if !text_content.is_empty() {
            if alert_id.is_none() {
                alert_id = self.alerts.get_active_id_by_user_id(user_id).await?;
            }

            if let Some(rid) = alert_id {
                self.alerts.update_description(rid, text_content).await?;
                should_respond = true;
            }
        }

        // 5. Send response based on alert status
        if should_respond {
            if let Some(rid) = alert_id {
                self.send_next_step(msg.chat.id, rid, lang).await?;
            }
        }

        Ok(())
    }

    async fn process_photo(
        &self,
        msg: &Message,
        alert: &Alert,
        photo: &teloxide::types::PhotoSize,
    ) -> anyhow::Result<bool> {
        let file = self.bot.get_file(&photo.file.id).await?;
        let mut buffer = Vec::new();
        self.bot.download_file(&file.path, &mut buffer).await?;

        let id = get_unique_id()?;
        let photo_path = format!("alerts/{}.jpg", id);
        self.storage.write_file(&photo_path, buffer).await?;

        let _ = self
            .bot
            .set_message_reaction(msg.chat.id, msg.id)
            .reaction(vec![ReactionType::Emoji {
                emoji: "👀".to_string(),
            }])
            .await;

        let is_first = self.photos.add_to_alert(alert.id, &photo_path).await?;

        log::info!("File added to alert: {} for alert {}", photo_path, alert.id);

        Ok(is_first)
    }

    async fn send_next_step(
        &self,
        chat_id: ChatId,
        alert_id: i64,
        lang: &str,
    ) -> anyhow::Result<()> {
        let alert = match self.alerts.get_by_id(alert_id).await {
            Ok(Some(r)) => r,
            _ => return Ok(()),
        };

        let photo_count = self.photos.count_by_alert_id(alert_id).await.unwrap_or(0);

        let text = if photo_count == 0 {
            self.i18n.tr("alert-intro", lang, None)
        } else if alert.lat.is_none() {
            self.i18n.tr("alert-photo-received", lang, None)
        } else if alert.description.is_none() {
            self.i18n.tr("alert-location-received", lang, None)
        } else {
            self.i18n.tr("alert-completed", lang, None)
        };

        self.bot
            .send_message(chat_id, text)
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    async fn handle_command(&self, msg: &Message, cmd: Command, lang: &str) -> anyhow::Result<()> {
        match cmd {
            Command::Start => self.handle_start(msg, lang).await,
            Command::Alert => self.handle_alert(msg, lang).await,
            Command::Map => self.handle_map(msg, lang).await,
        }
    }

    async fn start_alert(
        &self,
        msg: &Message,
        force_new: bool,
        lang: &str,
    ) -> anyhow::Result<Alert> {
        let user = msg.from.as_ref();
        let user_id = user.map(|u| u.id.0 as i64).unwrap_or(0);
        let chat_id = msg.chat.id.0;
        let username = user.and_then(|u| u.username.clone());
        let location = msg.location();
        let lat = location.map(|l| l.latitude);
        let lon = location.map(|l| l.longitude);

        self.alerts
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

    async fn handle_start(&self, msg: &Message, lang: &str) -> anyhow::Result<()> {
        self.bot
            .send_message(msg.chat.id, self.i18n.tr("start-welcome", lang, None))
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    async fn handle_alert(&self, msg: &Message, lang: &str) -> anyhow::Result<()> {
        self.start_alert(msg, true, lang).await?;

        self.bot
            .send_message(msg.chat.id, self.i18n.tr("alert-intro", lang, None))
            .parse_mode(ParseMode::Html)
            .await?;

        Ok(())
    }

    async fn handle_map(&self, msg: &Message, lang: &str) -> anyhow::Result<()> {
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
    alerts: Arc<AlertRepository>,
    photos: Arc<AlertPhotoRepository>,
    trees: Arc<TreeRepository>,
    storage: Arc<S3FileStorage>,
) {
    let bot = Bot::new(token.clone());
    let me = match bot.get_me().await {
        Ok(me) => me,
        Err(e) => {
            log::error!("Failed to get bot identity: {:?}", e);
            return;
        }
    };

    let chatbot = Arc::new(Chatbot::new(
        token,
        me.id,
        me.username().to_string(),
        i18n,
        alerts,
        photos,
        trees,
        storage,
    ));
    chatbot.run().await;
}
