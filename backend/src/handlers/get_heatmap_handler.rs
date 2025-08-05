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
        let (after, before) = Self::get_dates();
        let res = self.db.get_heatmap(after, before).await?;
        Ok(Self::format_data(res))
    }

    pub async fn handle_user(&self, user_id: u64) -> Result<Vec<HeatmapResponse>> {
        let (after, before) = Self::get_dates();
        let res = self.db.get_user_heatmap(after, before, user_id).await?;
        Ok(Self::format_data(res))
    }

    fn format_data(data: Vec<(String, u64)>) -> Vec<HeatmapResponse> {
        data.into_iter()
            .map(|(date, value)| HeatmapResponse {
                date: date.to_string(),
                value,
            })
            .collect()
    }

    fn get_dates() -> (u64, u64) {
        let today = Self::get_today();
        let after = today - 364 * 86400; // 364 days for full 52 weeks
        let before = today + 86400; // include data for today
        (after, before)
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
