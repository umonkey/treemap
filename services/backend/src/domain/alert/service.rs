//! Alert description editing.
//!
//! Alerts can be created from the web UI. This service only handles the
//! editable parts of an existing alert.

use super::{Alert, AlertRepository};
use crate::domain::iam::IamService;
use crate::services::{Context, Injectable};
use crate::types::{Error, Result};
use std::sync::Arc;

const MAX_DESCRIPTION_LEN: usize = 2000;

pub struct AlertService {
    alerts: Arc<AlertRepository>,
    iam: Arc<IamService>,
}

impl AlertService {
    /// Replace the description of an alert the user is allowed to edit.
    pub async fn update_description(
        &self,
        alert_id: u64,
        user_id: u64,
        description: &str,
    ) -> Result<Alert> {
        let mut alert = self
            .alerts
            .get(alert_id)
            .await?
            .ok_or(Error::AlertNotFound)?;

        let rights = self.iam.get_user_rights(user_id).await?;

        if !alert.is_editable(user_id, &rights) {
            return Err(Error::AccessDenied);
        }

        let description = normalize_description(description)?;

        self.alerts
            .update_description(alert_id, &description)
            .await?;

        alert.description = Some(description);

        Ok(alert)
    }
}

impl Injectable for AlertService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            alerts: Arc::new(ctx.build::<AlertRepository>()?),
            iam: Arc::new(ctx.build::<IamService>()?),
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
