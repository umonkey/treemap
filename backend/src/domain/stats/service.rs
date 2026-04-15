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

    pub async fn get_top_streets(&self) -> Result<Vec<StreetStatsResponse>> {
        let res = self.db.get_top_streets(1000).await?;

        let res = res
            .iter()
            .map(|(species, count)| StreetStatsResponse {
                address: species.clone(),
                count: *count,
            })
            .collect();

        Ok(res)
    }
}

impl Injectable for StatsService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}
