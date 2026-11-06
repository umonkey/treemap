//! Alert submission and photo processing.
//!
//! Alerts can be created from the web UI. The uploaded files are converted
//! to oriented JPEGs by queue handlers, so the request itself only validates
//! metadata and enqueues the work.

use super::{Alert, AlertRepository};
use crate::domain::alert_photo::AlertPhotoRepository;
use crate::domain::upload::UploadRepository;
use crate::infra::queue::Queue;
use crate::infra::storage::FileBucket;
use crate::services::queue_consumer::AddAlertPhotoMessage;
use crate::services::{Context, Injectable, ThumbnailerService};
use crate::types::{Error, Result};
use log::{error, info};
use std::sync::Arc;

const MAX_ALERT_PHOTOS: usize = 10;
const MAX_DESCRIPTION_LEN: usize = 2000;
const MAX_ALERT_PHOTO_SIZE: u32 = 2000;

pub struct AlertService {
    alerts: Arc<AlertRepository>,
    photos: Arc<AlertPhotoRepository>,
    uploads: Arc<UploadRepository>,
    storage: Arc<FileBucket>,
    thumbnailer: Arc<ThumbnailerService>,
    queue: Arc<Queue>,
}

impl AlertService {
    /// Create a web alert from validated metadata and uploaded file ids.
    ///
    /// A single queue message is sent per photo. The message handler writes
    /// the normalized JPEG and records its path.
    pub async fn create(
        &self,
        user_id: u64,
        lat: f64,
        lon: f64,
        description: String,
        files: Vec<String>,
    ) -> Result<Alert> {
        let description = description.trim().to_string();

        if description.is_empty() {
            return Err(Error::BadRequestMessage(
                "Alert description is required.".to_string(),
            ));
        }

        if description.chars().count() > MAX_DESCRIPTION_LEN {
            return Err(Error::BadRequestMessage(format!(
                "Alert description must be at most {MAX_DESCRIPTION_LEN} characters."
            )));
        }

        if !lat.is_finite()
            || !lon.is_finite()
            || !(-90.0..=90.0).contains(&lat)
            || !(-180.0..=180.0).contains(&lon)
        {
            return Err(Error::BadRequestMessage(
                "Alert coordinates are out of range.".to_string(),
            ));
        }

        if files.is_empty() {
            return Err(Error::BadRequestMessage(
                "At least one photo is required.".to_string(),
            ));
        }

        if files.len() > MAX_ALERT_PHOTOS {
            return Err(Error::BadRequestMessage(format!(
                "At most {MAX_ALERT_PHOTOS} photos are allowed."
            )));
        }

        let mut file_ids: Vec<u64> = Vec::new();

        for file in &files {
            let file_id: u64 = file
                .parse()
                .map_err(|_| Error::BadRequestMessage(format!("Invalid file id: {file}")))?;

            if !file_ids.contains(&file_id) {
                file_ids.push(file_id);
            }
        }

        for file_id in &file_ids {
            let upload = self
                .uploads
                .get(*file_id)
                .await?
                .ok_or(Error::FileNotFound)?;

            if upload.uploaded_at.is_none() {
                return Err(Error::BadRequestMessage(
                    "File upload is not complete.".to_string(),
                ));
            }

            if upload.added_by != user_id {
                return Err(Error::AccessDenied);
            }
        }

        let alert = self.alerts.create(user_id, lat, lon, &description).await?;

        for file_id in &file_ids {
            let message = AddAlertPhotoMessage {
                alert_id: alert.id,
                file_id: *file_id,
            };

            self.queue.push(&message.encode()).await?;
        }

        info!(
            "Alert {} created by user {user_id} with {} photos.",
            alert.id,
            file_ids.len()
        );

        Ok(alert)
    }

    /// Convert an uploaded image to JPEG and attach it to an alert.
    ///
    /// The deterministic path makes retries idempotent: the same upload always
    /// maps to the same file and photo record.
    pub async fn add_alert_photo(&self, alert_id: u64, file_id: u64) -> Result<()> {
        self.alerts
            .get(alert_id)
            .await?
            .ok_or_else(|| Error::DatabaseStructure(format!("Alert {alert_id} not found.")))?;

        let path = format!("alerts/{alert_id}/{file_id}.jpg");

        if self.photos.has_photo(alert_id, &path).await? {
            info!("Alert photo {path} is already recorded, skipping.");
            return Ok(());
        }

        self.uploads
            .get(file_id)
            .await?
            .ok_or(Error::FileNotFound)?;

        let body = self.storage.read_file(file_id).await?;

        let jpeg = match self.thumbnailer.to_jpeg(&body, MAX_ALERT_PHOTO_SIZE) {
            Ok(value) => value,

            Err(Error::BadImage) => {
                error!("Could not decode file {file_id} for alert {alert_id}, dropping it.");
                return Ok(());
            }

            Err(e) => return Err(e),
        };

        self.storage.write_file_at(&path, &jpeg, true).await?;
        self.photos.add(alert_id, &path).await?;

        info!("Alert photo {path} written, {} bytes.", jpeg.len());

        Ok(())
    }
}

impl Injectable for AlertService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            alerts: Arc::new(ctx.build::<AlertRepository>()?),
            photos: Arc::new(ctx.build::<AlertPhotoRepository>()?),
            uploads: Arc::new(ctx.build::<UploadRepository>()?),
            storage: ctx.storage(),
            thumbnailer: Arc::new(ctx.build::<ThumbnailerService>()?),
            queue: ctx.queue(),
        })
    }
}
