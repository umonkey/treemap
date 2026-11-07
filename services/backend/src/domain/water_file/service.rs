//! Attach files to water sources.

use super::models::WaterFile;
use super::repository::WaterFileRepository;
use crate::domain::file::FileRepository;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

pub struct WaterFileService {
    files: Arc<FileRepository>,
    water_files: Arc<WaterFileRepository>,
}

impl WaterFileService {
    pub async fn attach(
        &self,
        water_source_id: u64,
        user_id: u64,
        file_ids: Vec<u64>,
    ) -> Result<()> {
        for file_id in &file_ids {
            let file = self.files.get(*file_id).await?.ok_or(Error::FileNotFound)?;

            if file.is_deleted() {
                return Err(Error::FileNotFound);
            }

            if file.created_by != user_id {
                return Err(Error::AccessDenied);
            }

            self.water_files
                .add(&WaterFile {
                    id: get_unique_id()?,
                    water_source_id,
                    file_id: *file_id,
                    added_at: get_timestamp(),
                    added_by: user_id,
                })
                .await?;
        }

        info!(
            "Attached {} file(s) to water source {water_source_id} for user {user_id}.",
            file_ids.len()
        );

        Ok(())
    }
}

impl Injectable for WaterFileService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            files: Arc::new(ctx.build::<FileRepository>()?),
            water_files: Arc::new(ctx.build::<WaterFileRepository>()?),
        })
    }
}
