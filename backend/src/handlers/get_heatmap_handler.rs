//! Return the heatmap data for the updates.

use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

pub struct GetHeatmapHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetHeatmapHandler {
    pub async fn handle(&self) -> Result<Vec<HeatmapResponse>> {
        let today = Self::get_today();

        let after = today - 364 * 86400; // 364 days for full 52 weeks
        let before = today + 86400; // include data for today

        let res = self.db.get_heatmap(after, before).await?;

        let res = res
            .iter()
            .map(|(date, value)| HeatmapResponse {
                date: date.clone(),
                value: *value,
            })
            .collect();

        Ok(res)
    }

    fn get_today() -> u64 {
        let now = get_timestamp();
        now - (now % 86400)
    }
}

impl Locatable for GetHeatmapHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
