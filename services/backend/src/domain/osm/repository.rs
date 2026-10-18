use super::models::OsmTreeRecord;
use crate::infra::database::{Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::Result;
use log::error;
use std::sync::Arc;

const TABLE: &str = "osm_trees";

pub struct OsmTreeRepository {
    db: Arc<Database>,
}

impl OsmTreeRepository {
    pub async fn get(&self, id: u64) -> Result<Option<OsmTreeRecord>> {
        let query = SelectQuery::new(TABLE).with_condition("id", Value::from(id as i64));

        match self.db.get_record(query).await {
            Ok(Some(props)) => Ok(Some(OsmTreeRecord::from_attributes(&props)?)),
            Ok(None) => Ok(None),
            Err(err) => {
                error!("Error reading a tree: {err}");
                Err(err)
            }
        }
    }

    pub async fn add(&self, tree: &OsmTreeRecord) -> Result<()> {
        let query = InsertQuery::new(TABLE).with_values(tree.to_attributes());

        self.db.add_record(query).await?;

        Ok(())
    }

    pub async fn update(&self, tree: &OsmTreeRecord) -> Result<u64> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("id", Value::from(tree.id as i64))
            .with_values(tree.to_attributes());

        self.db.update(query).await.map_err(|e| {
            error!("Error updating an OSM tree: {e}");
            e
        })
    }

    pub async fn all_in_bbox(&self, bbox: [f64; 4]) -> Result<Vec<OsmTreeRecord>> {
        let [west, south, east, north] = bbox;
        let sql =
            format!("SELECT * FROM `{TABLE}` WHERE lat BETWEEN ? AND ? AND lon BETWEEN ? AND ?");

        let records = self
            .db
            .fetch_sql(
                &sql,
                &[
                    Value::from(south),
                    Value::from(north),
                    Value::from(west),
                    Value::from(east),
                ],
            )
            .await?;

        records.iter().map(OsmTreeRecord::from_attributes).collect()
    }

    pub async fn mark_invisible_before_in_bbox(
        &self,
        timestamp: u64,
        bbox: [f64; 4],
    ) -> Result<()> {
        let [west, south, east, north] = bbox;
        let sql = format!(
            "UPDATE `{TABLE}` SET visible = 0 \
             WHERE last_seen_at < ? AND lat BETWEEN ? AND ? AND lon BETWEEN ? AND ?"
        );

        self.db
            .execute_sql(
                &sql,
                &[
                    Value::from(timestamp as i64),
                    Value::from(south),
                    Value::from(north),
                    Value::from(west),
                    Value::from(east),
                ],
            )
            .await?;

        Ok(())
    }

    pub async fn get_nodes_to_delete(&self, limit: u64) -> Result<Vec<OsmTreeRecord>> {
        let sql = format!(
            "SELECT ot.* FROM `{TABLE}` ot \
             JOIN trees t ON ot.id = t.osm_id \
             WHERE (t.state = 'gone' OR t.state = 'replaced') \
             AND ot.visible = 1 \
             ORDER BY ot.id ASC \
             LIMIT ?"
        );

        let rows = self
            .db
            .fetch_sql(&sql, &[Value::from(limit as i64)])
            .await?;
        rows.iter().map(OsmTreeRecord::from_attributes).collect()
    }

    pub async fn mark_as_deleted(&self, ids: &[u64]) -> Result<()> {
        if ids.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "UPDATE `{TABLE}` SET visible = 0, version = version + 1 WHERE id IN ({})",
            placeholders.join(", ")
        );
        let params: Vec<Value> = ids.iter().map(|id| Value::from(*id as i64)).collect();

        self.db.execute_sql(&sql, &params).await?;

        Ok(())
    }
}

impl Injectable for OsmTreeRepository {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self { db: ctx.database() })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::{AppState, ContextExt};

    async fn setup() -> (AppState, Arc<OsmTreeRepository>) {
        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        state
            .database()
            .execute_sql(
                "INSERT INTO osm_trees \
                 (id, lat, lon, version, timestamp, user_id, user_name, visible, last_seen_at) \
                 VALUES \
                 (1, 40.1, 44.1, 1, '2024-01-01T00:00:00Z', 1, 'Mapper', 1, 100), \
                 (2, 50.0, 50.0, 1, '2024-01-01T00:00:00Z', 1, 'Mapper', 1, 100)",
                &[],
            )
            .await
            .expect("Error inserting OSM trees.");

        let repository = Arc::new(
            state
                .build::<OsmTreeRepository>()
                .expect("Error creating OsmTreeRepository"),
        );

        (state, repository)
    }

    #[tokio::test]
    async fn test_bbox_scoped_operations() {
        let (_state, repository) = setup().await;
        let bbox = [44.0, 40.0, 44.5, 40.5];

        repository
            .mark_invisible_before_in_bbox(200, bbox)
            .await
            .expect("Error marking OSM trees invisible.");

        let inside = repository
            .all_in_bbox(bbox)
            .await
            .expect("Error reading OSM trees in bbox.");

        assert_eq!(inside.len(), 1);
        assert_eq!(inside[0].id, 1);
        assert!(!inside[0].visible);

        let outside = repository
            .get(2)
            .await
            .expect("Error reading the outside OSM tree.")
            .expect("The outside OSM tree is missing.");

        assert!(outside.visible);
        assert_eq!(outside.last_seen_at, Some(100));
    }
}
