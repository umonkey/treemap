use super::models::WaterSource;
use super::schemas::Bounds;
use crate::infra::database::{Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

const TABLE: &str = "water_source";

pub struct WaterRepository {
    db: Arc<Database>,
}

impl WaterRepository {
    pub async fn get(&self, id: u64) -> Result<Option<WaterSource>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));
        self.query_single(query).await
    }

    pub async fn get_by_bounds(&self, bounds: Bounds) -> Result<Vec<WaterSource>> {
        let query =
            "SELECT * FROM `water_source` WHERE `lat` <= ? AND lat >= ? AND lon <= ? AND lon >= ?";

        let params = &[
            Value::from(bounds.n),
            Value::from(bounds.s),
            Value::from(bounds.e),
            Value::from(bounds.w),
        ];

        self.fetch(query, params).await
    }

    pub async fn add(&self, source: &WaterSource) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(source.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn update(&self, source: &WaterSource) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(source.id as i64))
            .with_values(source.to_attributes())
            .with_value("updated_at", Value::from(get_timestamp() as i64));

        self.db.update(query).await?;

        Ok(())
    }

    async fn query_single(&self, query: SelectQuery) -> Result<Option<WaterSource>> {
        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(WaterSource::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    async fn query_multiple(&self, query: SelectQuery) -> Result<Vec<WaterSource>> {
        let records = self.db.get_records(query).await?;

        records.iter().map(WaterSource::from_attributes).collect()
    }

    async fn fetch(&self, sql: &str, params: &[Value]) -> Result<Vec<WaterSource>> {
        let rows = self.db.fetch_sql(sql, params).await?;

        rows.iter().map(WaterSource::from_attributes).collect()
    }
}

impl Injectable for WaterRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::water::WaterStatus;
    use crate::services::AppState;
    use crate::services::ContextExt;

    async fn setup() -> Arc<WaterRepository> {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<WaterRepository>()
                .expect("Error creating water repository."),
        )
    }

    fn source(id: u64, lat: f64, lon: f64) -> WaterSource {
        WaterSource {
            id,
            created_at: 0,
            created_by: 1,
            updated_at: 0,
            lat,
            lon,
            status: WaterStatus::Operational,
        }
    }

    #[tokio::test]
    async fn test_add_and_get() {
        let repo = setup().await;

        repo.add(&source(1, 40.0, 44.0))
            .await
            .expect("Error adding a water source.");

        let res = repo.get(1).await.expect("Error getting water source.");
        assert!(res.is_some());
        assert_eq!(res.unwrap().lat, 40.0);
    }

    #[tokio::test]
    async fn test_get_by_bounds() {
        let repo = setup().await;

        repo.add(&source(1, 20.0, 20.0))
            .await
            .expect("Error adding a water source.");

        // (1) The source is within bounds and should be returned.
        let res = repo
            .get_by_bounds(Bounds {
                n: 40.0,
                e: 40.0,
                s: 0.0,
                w: 0.0,
            })
            .await
            .expect("Error getting water sources.");

        assert_eq!(1, res.len());

        // (2) The source is outside bounds and the result should be empty.
        let res = repo
            .get_by_bounds(Bounds {
                n: 40.0,
                e: 40.0,
                s: 30.0,
                w: 30.0,
            })
            .await
            .expect("Error getting water sources.");

        assert_eq!(0, res.len());
    }

    #[tokio::test]
    async fn test_update() {
        let repo = setup().await;

        repo.add(&source(1, 40.0, 44.0))
            .await
            .expect("Error adding a water source.");

        let updated = WaterSource {
            lat: 41.0,
            lon: 45.0,
            ..source(1, 40.0, 44.0)
        };

        repo.update(&updated)
            .await
            .expect("Error updating water source.");

        let res = repo
            .get(1)
            .await
            .expect("Error getting water source.")
            .unwrap();
        assert_eq!(res.lat, 41.0);
        assert_eq!(res.lon, 45.0);
    }
}
