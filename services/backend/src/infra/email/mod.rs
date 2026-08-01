use crate::infra::secrets::Secrets;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::error;

pub struct EmailClient {
    transport: SmtpTransport,
    from_email: String,
}

impl EmailClient {
    pub fn new(secrets: &Secrets) -> Result<Self> {
        let host = secrets
            .smtp_host
            .clone()
            .filter(|h| !h.is_empty())
            .ok_or_else(|| Error::Config("SMTP host not set".to_string()))?;

        let port = secrets.smtp_port.unwrap_or(587);

        let user = secrets.smtp_user.clone();

        let secret = secrets.smtp_secret.clone();

        let from_email = secrets
            .from_email
            .clone()
            .unwrap_or_else(|| "noreply@treemap.am".to_string());

        let mut builder = SmtpTransport::starttls_relay(&host)
            .map_err(|e| Error::Config(format!("Invalid SMTP host '{host}': {e}")))?
            .port(port);

        if let (Some(u), Some(s)) = (user, secret) {
            let creds = Credentials::new(u, s);
            builder = builder.credentials(creds);
        }

        let transport = builder.build();

        Ok(Self {
            transport,
            from_email,
        })
    }

    pub fn send(&self, to: &str, subject: &str, html_body: &str, text_body: &str) -> Result<()> {
        let email = Message::builder()
            .from(self.from_email.parse().map_err(|e| {
                error!("Invalid from email '{}': {e}", self.from_email);
                Error::Config(format!("Invalid from email: {e}"))
            })?)
            .to(to.parse().map_err(|e| {
                error!("Invalid to email '{to}': {e}");
                Error::Config(format!("Invalid to email: {e}"))
            })?)
            .subject(subject)
            .multipart(
                lettre::message::MultiPart::alternative()
                    .singlepart(lettre::message::SinglePart::plain(text_body.to_string()))
                    .singlepart(lettre::message::SinglePart::html(html_body.to_string())),
            )
            .map_err(|e| {
                error!("Failed to build email: {e}");
                Error::Config(format!("Failed to build email: {e}"))
            })?;

        self.transport.send(&email).map_err(|e| {
            error!("Failed to send email to {to}: {e}");
            Error::Config(format!("Failed to send email: {e}"))
        })?;

        Ok(())
    }
}

impl Injectable for EmailClient {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Self::new(ctx.secrets().as_ref())
    }
}
