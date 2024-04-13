use log::info;
use std::sync::Arc;

use crate::Result;
use crate::utils::{get_timestamp, get_unique_id};
use crate::services::Database;
use crate::types::UploadTicket;

pub struct UploadService {
    db: Arc<dyn Database>,
}

impl UploadService {
    pub async fn init(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn create_ticket(&self, user_id: u64) -> Result<UploadTicket> {
        let ticket = UploadTicket {
            id: get_unique_id()?,
            created_at: get_timestamp(),
            created_by: user_id,
            upload_url: "foobar".to_string(),
        };

        self.db.add_upload_ticket(&ticket).await?;

        info!("Upload ticket {} created for user {}.", ticket.id, user_id);

        Ok(ticket)
    }
}
