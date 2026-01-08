//! Return the heatmap data for the updates.

use crate::infra::database::Database;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use chrono::DateTime;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GetHeatmapHandler {
    db: Arc<Database>,
}

impl GetHeatmapHandler {
    pub async fn handle(&self) -> Result<Vec<HeatmapResponse>> {
        let (after, before) = Self::get_dates();
        let res = self.db.get_heatmap(after, before).await?;
        Ok(Self::format_data(after, before, res))
    }

    pub async fn handle_user(&self, user_id: u64) -> Result<Vec<HeatmapResponse>> {
        let (after, before) = Self::get_dates();
        let res = self.db.get_user_heatmap(after, before, user_id).await?;
        Ok(Self::format_data(after, before, res))
    }

    fn format_data(after: u64, before: u64, data: Vec<(String, u64)>) -> Vec<HeatmapResponse> {
        let items = data
            .into_iter()
            .map(|(date, value)| HeatmapResponse { date, value })
            .collect();

        Self::normalize_heatmap(after, before, items)
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

    /// Make sure the heat map contains all dates, even when there's no activity.
    /// This makes sure that days after last activity are being displayed.
    fn normalize_heatmap(
        after: u64,
        before: u64,
        items: Vec<HeatmapResponse>,
    ) -> Vec<HeatmapResponse> {
        let mut map = Self::init_heatmap(after, before);

        for item in items {
            if map.contains_key(&item.date) {
                map.insert(item.date.clone(), item);
            }
        }

        map.into_values().collect()
    }

    /// Initialize an empty heatmap with all possible dates from the specified time range.
    fn init_heatmap(after: u64, before: u64) -> HashMap<String, HeatmapResponse> {
        let mut map: HashMap<String, HeatmapResponse> = HashMap::new();

        let mut timestamp = after;

        while timestamp <= before {
            if let Some(dt) = DateTime::from_timestamp(timestamp as i64, 0) {
                let date_str = dt.format("%Y-%m-%d").to_string();

                map.insert(
                    date_str.clone(),
                    HeatmapResponse {
                        date: date_str,
                        value: 0,
                    },
                );
            }

            timestamp += 86400; // increment by one day
        }

        map
    }
}

impl Locatable for GetHeatmapHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
