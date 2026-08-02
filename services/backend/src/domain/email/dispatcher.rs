use crate::domain::email::{Email, EmailRepository};
use crate::infra::email::EmailClient;
use crate::infra::secrets::Secrets;
use crate::types::{Error, Result};
use crate::utils::get_timestamp;
use handlebars::Handlebars;
use std::sync::Arc;

pub struct EmailDispatcher {
    client: Arc<EmailClient>,
    repo: Arc<EmailRepository>,
    handlebars: Handlebars<'static>,
}

impl EmailDispatcher {
    fn init_handlebars() -> Result<Handlebars<'static>> {
        let mut handlebars = Handlebars::new();
        let active_dir = "templates/email";

        for name in &[
            "panorama_transcoding_failed",
            "panorama_sync",
            "panorama_ready",
        ] {
            let html_path = format!("{}/{}.html.hbs", active_dir, name);
            let txt_path = format!("{}/{}.txt.hbs", active_dir, name);

            if std::path::Path::new(&html_path).exists() {
                handlebars
                    .register_template_file(&format!("{}_html", name), &html_path)
                    .map_err(|e| {
                        Error::Config(format!("Failed to register template {html_path}: {e}"))
                    })?;
            }
            if std::path::Path::new(&txt_path).exists() {
                handlebars
                    .register_template_file(&format!("{}_txt", name), &txt_path)
                    .map_err(|e| {
                        Error::Config(format!("Failed to register template {txt_path}: {e}"))
                    })?;
            }
        }

        Ok(handlebars)
    }

    pub fn new(secrets: &Secrets, repo: Arc<EmailRepository>) -> Result<Self> {
        let client = Arc::new(EmailClient::new(secrets)?);
        let handlebars = Self::init_handlebars()?;

        Ok(Self {
            client,
            repo,
            handlebars,
        })
    }

    pub async fn send_template(
        &self,
        to: &str,
        template_name: &str,
        data: &serde_json::Value,
    ) -> Result<()> {
        let html_key = format!("{}_html", template_name);
        let txt_key = format!("{}_txt", template_name);

        let html_body = self.handlebars.render(&html_key, data).map_err(|e| {
            log::error!("Failed to render HTML template {template_name}: {e}");
            Error::Config(format!("Template render error: {e}"))
        })?;

        let text_body = self.handlebars.render(&txt_key, data).map_err(|e| {
            log::error!("Failed to render TXT template {template_name}: {e}");
            Error::Config(format!("Template render error: {e}"))
        })?;

        let subject_string;
        let subject = match template_name {
            "panorama_transcoding_failed" => "Panorama Transcoding Failed",
            "panorama_sync" => "Panorama Ready for Sync",
            "panorama_ready" => {
                let name = data.get("name").and_then(|v| v.as_str()).unwrap_or("");
                subject_string = format!("Panorama ready: {name}");
                &subject_string
            }
            _ => "Notification from Tree Map",
        };

        self.client.send(to, subject, &html_body, &text_body)
    }

    pub async fn process_email(&self, email: &mut Email) -> Result<()> {
        let data: serde_json::Value = serde_json::from_str(&email.template_data)
            .map_err(|e| Error::Config(format!("Invalid template data JSON: {e}")))?;

        email.attempts += 1;

        match self
            .send_template(&email.recipient, &email.event_name, &data)
            .await
        {
            Ok(()) => {
                email.status = "SENT".to_string();
                email.sent_at = Some(get_timestamp());
                email.last_error = None;
                self.repo.update(email).await?;
                Ok(())
            }
            Err(e) => {
                email.status = "FAILED".to_string();
                email.last_error = Some(e.to_string());
                self.repo.update(email).await?;
                Err(e)
            }
        }
    }

    pub async fn process_pending(&self) -> Result<usize> {
        let emails = self.repo.get_pending_or_failed().await?;
        let count = emails.len();

        for mut email in emails {
            if let Err(e) = self.process_email(&mut email).await {
                log::error!("Failed to send email {}: {e}", email.id);
            }
        }

        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;

    #[tokio::test]
    async fn test_email_dispatcher_rendering() {
        let state = AppState::new().await.expect("Failed to create AppState");
        let email_repo = Arc::new(EmailRepository::new(state.database.clone()));
        let dispatcher = EmailDispatcher::new(&state.secrets, email_repo)
            .expect("Failed to create EmailDispatcher");
        let data = serde_json::json!({
            "panorama_id": 123,
            "reason": "Test failure reason"
        });

        let html_res = dispatcher
            .handlebars
            .render("panorama_transcoding_failed_html", &data);
        assert!(html_res.is_ok());
        let html = html_res.unwrap();
        assert!(html.contains("123"));
        assert!(html.contains("Test failure reason"));

        let txt_res = dispatcher
            .handlebars
            .render("panorama_transcoding_failed_txt", &data);
        assert!(txt_res.is_ok());
        let txt = txt_res.unwrap();
        assert!(txt.contains("123"));
        assert!(txt.contains("Test failure reason"));

        let ready_data = serde_json::json!({
            "panorama_id": 456,
            "name": "Test Panorama"
        });

        let ready_html_res = dispatcher
            .handlebars
            .render("panorama_ready_html", &ready_data);
        assert!(ready_html_res.is_ok());
        let ready_html = ready_html_res.unwrap();
        assert!(ready_html.contains("456"));
        assert!(ready_html.contains("Test Panorama"));

        let ready_txt_res = dispatcher
            .handlebars
            .render("panorama_ready_txt", &ready_data);
        assert!(ready_txt_res.is_ok());
        let ready_txt = ready_txt_res.unwrap();
        assert!(ready_txt.contains("456"));
        assert!(ready_txt.contains("Test Panorama"));
    }
}
