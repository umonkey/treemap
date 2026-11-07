//! Create web alerts with attached files.

use super::models::AlertFile;
use super::repository::AlertFileRepository;
use crate::domain::alert::{Alert, AlertRepository};
use crate::domain::file::{file_url, FileRepository};
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use crate::utils::{get_timestamp, get_unique_id};
use log::info;
use std::sync::Arc;

const MAX_ALERT_PHOTOS: usize = 10;
const MAX_DESCRIPTION_LEN: usize = 2000;

pub struct AlertFileService {
    alerts: Arc<AlertRepository>,
    alert_files: Arc<AlertFileRepository>,
    files: Arc<FileRepository>,
}

impl AlertFileService {
    /// Create a web alert and attach the caller's files.
    pub async fn create_alert(
        &self,
        user_id: u64,
        lat: f64,
        lon: f64,
        description: String,
        files: Vec<String>,
    ) -> Result<Alert> {
        let description = normalize_description(&description)?;

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
            let file = self.files.get(*file_id).await?.ok_or(Error::FileNotFound)?;

            if file.is_deleted() {
                return Err(Error::FileNotFound);
            }

            if file.created_by != user_id {
                return Err(Error::AccessDenied);
            }
        }

        let alert = self.alerts.create(user_id, lat, lon, &description).await?;

        for file_id in &file_ids {
            self.alert_files
                .add(&AlertFile {
                    id: get_unique_id()?,
                    alert_id: alert.id,
                    file_id: *file_id,
                    added_at: get_timestamp(),
                    added_by: user_id,
                })
                .await?;
        }

        info!(
            "Alert {} created by user {user_id} with {} photos.",
            alert.id,
            file_ids.len()
        );

        Ok(alert)
    }

    /// Return public URLs for the visible files attached to an alert.
    pub async fn get_urls(&self, alert_id: u64, base_url: &str) -> Result<Vec<String>> {
        let links = self.alert_files.find_by_alert(alert_id).await?;

        let mut urls = Vec::new();

        for link in links {
            if let Some(details) = self.files.get_details(link.file_id).await? {
                if !details.is_listable() {
                    continue;
                }

                if let Some(variant) = details.original().or_else(|| details.small()) {
                    urls.push(file_url(base_url, &variant.key));
                }
            }
        }

        Ok(urls)
    }
}

impl Injectable for AlertFileService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            alerts: Arc::new(ctx.build::<AlertRepository>()?),
            alert_files: Arc::new(ctx.build::<AlertFileRepository>()?),
            files: Arc::new(ctx.build::<FileRepository>()?),
        })
    }
}

fn normalize_description(description: &str) -> Result<String> {
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

    Ok(description)
}
