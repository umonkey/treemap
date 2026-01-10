use super::schemas::*;
use crate::infra::database::Database;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct StatsService {
    db: Arc<Database>,
}

impl StatsService {
    pub async fn count_trees_by_state(&self) -> Result<Vec<StateStatsResponse>> {
        let res = self.db.get_state_stats().await?;

        let res = res
            .iter()
            .map(|(state, count)| StateStatsResponse {
                state: state.clone(),
                count: *count,
            })
            .collect();

        Ok(res)
    }
}

impl Locatable for StatsService {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
