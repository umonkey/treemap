pub mod dispatcher;
mod models;
mod repository;

pub use dispatcher::EmailDispatcher;
pub use models::Email;
pub use repository::EmailRepository;

use crate::services::{Context, Injectable};
use crate::types::Result;
use crate::utils::get_timestamp;
use std::sync::Arc;

pub struct EmailService {
    pub repo: Arc<EmailRepository>,
}

impl EmailService {
    pub fn new(repo: Arc<EmailRepository>) -> Self {
        Self { repo }
    }

    pub async fn enqueue(
        &self,
        recipient: &str,
        event_name: &str,
        data: &serde_json::Value,
    ) -> Result<()> {
        let email = Email {
            id: 0,
            recipient: recipient.to_string(),
            event_name: event_name.to_string(),
            template_data: data.to_string(),
            status: "PENDING".to_string(),
            attempts: 0,
            last_error: None,
            created_at: get_timestamp(),
            sent_at: None,
        };
        self.repo.add(&email).await
    }
}

impl Injectable for EmailService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let repo = Arc::new(ctx.build::<EmailRepository>()?);
        Ok(Self::new(repo))
    }
}
