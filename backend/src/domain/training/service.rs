use super::models::TrainingRecord;
use super::repository::TrainingRepository;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use std::sync::Arc;

pub struct TrainingService {
    repo: Arc<TrainingRepository>,
}

impl TrainingService {
    pub async fn add(&self, user_id: u64, result: f64) -> Result<()> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let rec = TrainingRecord {
            id,
            user_id,
            added_at: now,
            result,
        };

        self.repo.add(&rec).await?;

        Ok(())
    }
}

impl Locatable for TrainingService {
    fn create(locator: &Locator) -> Result<Self> {
        let repo = locator.get::<TrainingRepository>()?;
        Ok(Self { repo })
    }
}
