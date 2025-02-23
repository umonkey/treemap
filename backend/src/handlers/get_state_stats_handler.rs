use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetStateStatsHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetStateStatsHandler {
    pub async fn handle(&self) -> Result<Vec<StateStatsResponse>> {
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

impl Locatable for GetStateStatsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
