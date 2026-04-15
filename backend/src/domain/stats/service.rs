use super::schemas::*;
use crate::infra::database::{Database, Value};
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct StatsService {
    db: Arc<Database>,
}

impl StatsService {
    pub async fn count_trees_by_state(&self) -> Result<Vec<StateStatsResponse>> {
        let rows = self
            .db
            .fetch_sql(
                "SELECT state, COUNT(1) AS cnt FROM trees WHERE state IS NOT NULL GROUP BY state ORDER BY cnt DESC",
                &[],
            )
            .await?;

        let mut res = Vec::new();

        for row in rows {
            let state = row.require_string("state")?;
            let count = row.require_u64("cnt")?;
            res.push(StateStatsResponse { state, count });
        }

        Ok(res)
    }

    pub async fn get_top_streets(&self) -> Result<Vec<StreetStatsResponse>> {
        let rows = self
            .db
            .fetch_sql(
                "SELECT address, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND address IS NOT NULL GROUP BY LOWER(address) ORDER BY cnt DESC, LOWER(address) LIMIT ?1",
                &[Value::from(1000u64)],
            )
            .await?;

        let mut res = Vec::new();

        for row in rows {
            let address = row.require_string("address")?;
            let count = row.require_u64("cnt")?;
            res.push(StreetStatsResponse { address, count });
        }

        Ok(res)
    }
}

impl Injectable for StatsService {
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

    async fn setup() -> StatsService {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new().await.expect("Error creating app state.")
            .session().await
            .expect("Error creating session state.");

        state
            .build::<StatsService>()
            .expect("Error creating StatsService")
    }

    #[tokio::test]
    async fn test_count_trees_by_state() {
        let service = setup().await;

        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .expect("Error clearing trees.");

        service
            .db
            .execute_sql("INSERT INTO trees (id, lat, lon, species, state, added_at, updated_at, updated_by, added_by) VALUES (1, 40.1, 44.1, 'Birch', 'healthy', 0, 0, 1, 1)", &[])
            .await
            .expect("Error adding tree.");

        let res = service
            .count_trees_by_state()
            .await
            .expect("Error getting stats.");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].state, "healthy");
        assert_eq!(res[0].count, 1);
    }

    #[tokio::test]
    async fn test_get_top_streets() {
        let service = setup().await;

        service
            .db
            .execute_sql("DELETE FROM trees", &[])
            .await
            .expect("Error clearing trees.");

        service
            .db
            .execute_sql("INSERT INTO trees (id, lat, lon, species, address, state, added_at, updated_at, updated_by, added_by) VALUES (1, 40.1, 44.1, 'Birch', 'Main St', 'healthy', 0, 0, 1, 1)", &[])
            .await
            .expect("Error adding tree.");

        let res = service
            .get_top_streets()
            .await
            .expect("Error getting stats.");

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].address, "Main St");
        assert_eq!(res[0].count, 1);
    }
}
