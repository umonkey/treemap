use anyhow::Result;
use teloxide::prelude::*;
use teloxide::types::{ChatId, InputFile, InputMedia, InputMediaPhoto, MessageId, ThreadId};

pub struct TelegramClient {
    bot: Bot,
}

impl TelegramClient {
    pub fn new(token: String) -> Self {
        Self {
            bot: Bot::new(token),
        }
    }

    pub async fn send_message(
        &self,
        chat_id: i64,
        thread_id: Option<i64>,
        text: &str,
        urls: &[String],
    ) -> Result<()> {
        let thread_id = thread_id.map(|id| ThreadId(MessageId(id as i32)));

        if urls.is_empty() {
            let mut req = self.bot.send_message(ChatId(chat_id), text.to_string());
            if let Some(t) = thread_id {
                req = req.message_thread_id(t);
            }
            req.await?;
        } else if urls.len() == 1 {
            let mut req = self
                .bot
                .send_photo(ChatId(chat_id), InputFile::url(urls[0].parse()?))
                .caption(text.to_string());
            if let Some(t) = thread_id {
                req = req.message_thread_id(t);
            }
            req.await?;
        } else {
            let mut media = Vec::new();
            for (i, url) in urls.iter().take(10).enumerate() {
                let mut photo = InputMediaPhoto::new(InputFile::url(url.parse()?));
                if i == 0 {
                    photo = photo.caption(text.to_string());
                }
                media.push(InputMedia::Photo(photo));
            }
            let mut req = self.bot.send_media_group(ChatId(chat_id), media);
            if let Some(t) = thread_id {
                req = req.message_thread_id(t);
            }
            req.await?;
        }

        Ok(())
    }
}

/// Parses a recipient string of the form `chat_id` or `chat_id:topic_id`.
pub fn parse_recipient(recipient: &str) -> Result<(i64, Option<i64>)> {
    match recipient.split_once(':') {
        Some((chat, topic)) => {
            let topic_id = topic.trim().parse::<i64>().map_err(|e| {
                anyhow::anyhow!("Invalid topic id in recipient '{}': {}", recipient, e)
            })?;
            let chat_id = chat.trim().parse::<i64>().map_err(|e| {
                anyhow::anyhow!("Invalid chat id in recipient '{}': {}", recipient, e)
            })?;
            Ok((chat_id, Some(topic_id)))
        }
        None => {
            let chat_id = recipient.trim().parse::<i64>().map_err(|e| {
                anyhow::anyhow!("Invalid chat id in recipient '{}': {}", recipient, e)
            })?;
            Ok((chat_id, None))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_plain_chat_id() {
        assert_eq!(parse_recipient("12345").unwrap(), (12345, None));
    }

    #[test]
    fn parses_chat_id_with_topic() {
        assert_eq!(parse_recipient("12345:678").unwrap(), (12345, Some(678)));
    }

    #[test]
    fn trims_whitespace() {
        assert_eq!(
            parse_recipient(" 12345 : 678 ").unwrap(),
            (12345, Some(678))
        );
    }

    #[test]
    fn rejects_invalid_chat_id() {
        assert!(parse_recipient("abc").is_err());
    }

    #[test]
    fn rejects_invalid_topic_id() {
        assert!(parse_recipient("12345:abc").is_err());
    }
}
