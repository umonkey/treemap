use crate::common::database::repositories::TrainingRepository;
use crate::services::*;
use crate::types::*;
use crate::utils::{get_timestamp, get_unique_id};
use std::sync::Arc;

pub struct AddTrainingHandler {
    repo: Arc<TrainingRepository>,
}

impl AddTrainingHandler {
    pub async fn handle(&self, req: AddTrainingRequest) -> Result<()> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let rec = TrainingRecord {
            id,
            user_id: req.user_id,
            added_at: now,
            result: req.result,
        };

        self.repo.add(&rec).await?;

        Ok(())
    }
}

impl Locatable for AddTrainingHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let repo = locator.get::<TrainingRepository>()?;
        Ok(Self { repo })
    }
}
