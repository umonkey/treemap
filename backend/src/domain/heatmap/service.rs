//! Return the heatmap data for the updates.

use super::schemas::*;
use crate::infra::database::{Database, Value};
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use chrono::DateTime;
use std::collections::HashMap;
use std::sync::Arc;

pub struct HeatmapService {
    db: Arc<Database>,
}

impl HeatmapService {
    pub async fn get_total(&self) -> Result<Vec<HeatmapItem>> {
        let (after, before) = Self::get_dates();

        let rows = self
            .db
            .sql(
                "SELECT DATE(added_at, 'unixepoch') AS date, COUNT(distinct tree_id) AS count FROM trees_props WHERE added_at >= ?1 AND added_at < ?2 GROUP BY date ORDER BY date",
                &[Value::from(after), Value::from(before)],
            )
            .await?;

        let mut data = Vec::new();

        for row in rows {
            let date = row.require_string("date")?;
            let count = row.require_u64("count")?;
            data.push((date, count));
        }

        Ok(Self::format_data(after, before, data))
    }

    pub async fn get_user(&self, user_id: u64) -> Result<Vec<HeatmapItem>> {
        let (after, before) = Self::get_dates();

        let rows = self
            .db
            .sql(
                "SELECT DATE(added_at, 'unixepoch') AS date, COUNT(distinct tree_id) AS count FROM trees_props WHERE added_at >= ?1 AND added_at < ?2 AND added_by = ?3 GROUP BY date ORDER BY date",
                &[
                    Value::from(after),
                    Value::from(before),
                    Value::from(user_id),
                ],
            )
            .await?;

        let mut data = Vec::new();

        for row in rows {
            let date = row.require_string("date")?;
            let count = row.require_u64("count")?;
            data.push((date, count));
        }

        Ok(Self::format_data(after, before, data))
    }

    fn format_data(after: u64, before: u64, data: Vec<(String, u64)>) -> Vec<HeatmapItem> {
        let items = data
            .into_iter()
            .map(|(date, value)| HeatmapItem { date, value })
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
    fn normalize_heatmap(after: u64, before: u64, items: Vec<HeatmapItem>) -> Vec<HeatmapItem> {
        let mut map = Self::init_heatmap(after, before);

        for item in items {
            if map.contains_key(&item.date) {
                map.insert(item.date.clone(), item);
            }
        }

        map.into_values().collect()
    }

    /// Initialize an empty heatmap with all possible dates from the specified time range.
    fn init_heatmap(after: u64, before: u64) -> HashMap<String, HeatmapItem> {
        let mut map: HashMap<String, HeatmapItem> = HashMap::new();

        let mut timestamp = after;

        while timestamp <= before {
            if let Some(dt) = DateTime::from_timestamp(timestamp as i64, 0) {
                let date_str = dt.format("%Y-%m-%d").to_string();

                map.insert(
                    date_str.clone(),
                    HeatmapItem {
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

impl Injectable for HeatmapService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::AppState;
    use crate::services::ContextExt;
    use log::debug;

    async fn setup() -> HeatmapService {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new().await.expect("Error creating app state.");

        state
            .build::<HeatmapService>()
            .expect("Error creating HeatmapService")
    }

    #[tokio::test]
    async fn test_get_total() {
        let service = setup().await;

        service
            .db
            .execute(include_str!(
                "../../infra/database/fixtures/test_heatmap.sql"
            ))
            .await
            .expect("Error adding heatmap input.");

        let res = service.get_total().await.expect("Error getting heatmap.");

        let day1 = res.iter().find(|i| i.date == "2025-08-04").unwrap();
        assert_eq!(day1.value, 1);

        let day2 = res.iter().find(|i| i.date == "2025-08-05").unwrap();
        assert_eq!(day2.value, 2);
    }

    #[tokio::test]
    async fn test_get_user() {
        let service = setup().await;

        service
            .db
            .execute(include_str!(
                "../../infra/database/fixtures/test_heatmap.sql"
            ))
            .await
            .expect("Error adding heatmap input.");

        let res = service.get_user(1).await.expect("Error getting heatmap.");

        let day1 = res.iter().find(|i| i.date == "2025-08-04").unwrap();
        assert_eq!(day1.value, 1);

        let day2 = res.iter().find(|i| i.date == "2025-08-05").unwrap();
        assert_eq!(day2.value, 1);
    }
}
