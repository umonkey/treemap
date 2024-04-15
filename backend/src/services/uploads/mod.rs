use log::info;
use std::sync::Arc;

use crate::services::{Database, S3Service};
use crate::types::{Result, UploadTicket};
use crate::utils::{get_timestamp, get_unique_id};

pub struct UploadService {
    db: Arc<dyn Database>,
    s3: S3Service,
}

impl UploadService {
    pub async fn init(db: &Arc<dyn Database>) -> Result<Self> {
        Ok(Self {
            db: db.clone(),
            s3: S3Service::init().await?,
        })
    }

    pub async fn create_ticket(&self, user_id: u64) -> Result<UploadTicket> {
        let id = get_unique_id()?;
        let upload_url = self.s3.get_upload_url(&id.to_string()).await?;

        let ticket = UploadTicket {
            id,
            created_at: get_timestamp(),
            created_by: user_id,
            upload_url,
        };

        self.db.add_upload_ticket(&ticket).await?;

        info!("Upload ticket {} created for user {}.", id, user_id);

        Ok(ticket)
    }
}