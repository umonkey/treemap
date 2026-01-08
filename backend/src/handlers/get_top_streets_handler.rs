use crate::infra::database::Database;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTopStreetsHandler {
    db: Arc<Database>,
}

impl GetTopStreetsHandler {
    pub async fn handle(&self) -> Result<Vec<StreetStatsResponse>> {
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

impl Locatable for GetTopStreetsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<Database>()?;
        Ok(Self { db })
    }
}
