//! Copies AWS Batch job logs into a panorama's storage folder.

use super::repository::PanoramaRepository;
use crate::infra::logs::LogsClient;
use crate::infra::storage::PanoramaBucket;
use crate::services::{Context, Injectable};
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct PanoramaLogs {
    panoramas: Arc<PanoramaBucket>,
    repo: Arc<PanoramaRepository>,
    logs: LogsClient,
}

impl PanoramaLogs {
    /// Fetches the job log and writes it to `{storage_key}/processing.log`.
    ///
    /// The write is idempotent, so a redelivered queue message overwrites the
    /// file instead of failing.
    pub async fn download(&self, id: u64, log_stream: &str) -> Result<()> {
        let panorama = self.repo.get(id).await?.ok_or(Error::PanoramaNotFound)?;

        let data = self.logs.fetch(log_stream).await?;
        let path = format!("{}/processing.log", panorama.storage_key);

        self.panoramas
            .write_file(&path, data.as_bytes(), true)
            .await?;

        info!(
            "Panorama {id} log stream {log_stream} written to {path}, {} bytes.",
            data.len()
        );

        Ok(())
    }
}

impl Injectable for PanoramaLogs {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        let config = ctx.config();
        let secrets = ctx.secrets();

        Ok(Self {
            panoramas: ctx.panoramas(),
            repo: Arc::new(ctx.build::<PanoramaRepository>()?),
            logs: LogsClient::new(&config, &secrets)?,
        })
    }
}
