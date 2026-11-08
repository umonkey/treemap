//! Snapshot the object storage inventory into SQLite.

use super::{StorageObject, StorageRepository};
use crate::infra::database::Database;
use crate::infra::storage::{
    BackupBucket, FileBucket, PanoramaBucket, PanoramaSourceBucket, StorageFile,
};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use log::info;
use std::sync::Arc;

/// Summary of one bucket sync.
pub struct BucketSyncResult {
    pub bucket: String,
    pub seen: u64,
    pub bytes: u64,
    pub missing: u64,
}

pub struct StorageSyncService {
    db: Arc<Database>,
    files: Arc<FileBucket>,
    backups: Arc<BackupBucket>,
    panoramas: Arc<PanoramaBucket>,
    panoramas_source: Arc<PanoramaSourceBucket>,
}

impl StorageSyncService {
    /// List every configured bucket and upsert the inventory.
    pub async fn sync(&self) -> Result<Vec<BucketSyncResult>> {
        let mut results = Vec::new();

        let bucket = self.files.name().to_string();
        let files = self.files.list_files("").await?;
        results.push(self.sync_bucket("files", &bucket, files).await?);

        let bucket = self.backups.name().to_string();
        let files = self.backups.list_files("").await?;
        results.push(self.sync_bucket("backups", &bucket, files).await?);

        let bucket = self.panoramas.name().to_string();
        let files = self.panoramas.list_files("").await?;
        results.push(self.sync_bucket("panoramas", &bucket, files).await?);

        let bucket = self.panoramas_source.name().to_string();
        let files = self.panoramas_source.list_files("").await?;
        results.push(self.sync_bucket("panoramas-source", &bucket, files).await?);

        Ok(results)
    }

    async fn sync_bucket(
        &self,
        role: &str,
        bucket: &str,
        files: Vec<StorageFile>,
    ) -> Result<BucketSyncResult> {
        let seen_at = get_timestamp();
        let total_bytes: u64 = files.iter().map(|file| file.size).sum();

        let objects: Vec<StorageObject> = files
            .into_iter()
            .map(|file| StorageObject {
                bucket: bucket.to_string(),
                key: file.path,
                size_bytes: file.size,
                etag: file.etag,
                last_modified_at: file.last_modified_at.unwrap_or(0),
            })
            .collect();

        let tx = Arc::new(self.db.transact().await?);
        let repo = StorageRepository::new(tx.clone());

        repo.mark_missing(bucket, seen_at).await?;
        repo.upsert(&objects, seen_at).await?;

        let missing = repo.count_missing(bucket).await?;

        tx.commit().await?;

        info!(
            "Storage sync {} (bucket {}): {} objects, {} bytes, {} missing.",
            role,
            bucket,
            objects.len(),
            total_bytes,
            missing
        );

        Ok(BucketSyncResult {
            bucket: bucket.to_string(),
            seen: objects.len() as u64,
            bytes: total_bytes,
            missing,
        })
    }
}

impl Injectable for StorageSyncService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            files: ctx.storage(),
            backups: ctx.backups(),
            panoramas: ctx.panoramas(),
            panoramas_source: ctx.panoramas_source(),
        })
    }
}
