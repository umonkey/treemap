use super::models::Species;
use crate::infra::database::{Attributes, Database, InsertQuery, SelectQuery, UpdateQuery, Value};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use log::error;
use std::collections::HashMap;
use std::sync::Arc;

const TABLE: &str = "species";
const SUGGEST_WINDOW: u64 = 3600 * 24; // 24 hours

pub struct SpeciesRepository {
    db: Arc<Database>,
}

impl SpeciesRepository {
    pub async fn search(&self, query: &str) -> Result<Vec<Species>> {
        let pattern = format!("%{}%", query.trim().to_lowercase());

        let rows = self
            .db
            .fetch_sql(
                "SELECT name, local, keywords, wikidata_id FROM species WHERE name LIKE ?1 OR local LIKE ?1 OR keywords LIKE ?1 ORDER BY name LIMIT 10",
                &[Value::from(pattern)],
            )
            .await?;

        rows.iter().map(Self::from_row).collect()
    }

    pub async fn all(&self) -> Result<Vec<Species>> {
        let query = SelectQuery::new(TABLE).with_order("name");
        let rows = self.db.get_records(query).await?;

        rows.iter().map(Self::from_row).collect()
    }

    pub async fn get(&self, name: &str) -> Result<Option<Species>> {
        let query = SelectQuery::new(TABLE).with_condition("name", Value::from(name.to_string()));

        match self.db.get_record(query).await? {
            Some(row) => Ok(Some(Self::from_row(&row)?)),
            None => Ok(None),
        }
    }

    pub async fn add(&self, species: &Species) -> Result<()> {
        let query = InsertQuery::new(TABLE)
            .with_value("name", Value::from(species.name.clone()))
            .with_value("local", Value::from(species.local.clone()))
            .with_value("keywords", Value::from(species.keywords.clone()))
            .with_value("wikidata_id", Value::from(species.wikidata_id.clone()));

        self.db.add_record(query).await.map_err(|e| {
            error!("Error creating a species: {e}");
            e
        })?;

        Ok(())
    }

    pub async fn update(&self, species: &Species) -> Result<()> {
        let query = UpdateQuery::new(TABLE)
            .with_condition("name", Value::from(species.name.clone()))
            .with_value("local", Value::from(species.local.clone()))
            .with_value("keywords", Value::from(species.keywords.clone()))
            .with_value("wikidata_id", Value::from(species.wikidata_id.clone()));

        self.db.update(query).await.map_err(|e| {
            error!("Error updating a species: {e}");
            e
        })?;

        Ok(())
    }

    pub async fn suggest(&self, user_id: u64) -> Result<Vec<String>> {
        let since = get_timestamp() - SUGGEST_WINDOW;

        let rows = self
            .db
            .fetch_sql(
                "SELECT species, COUNT(1) AS use_count FROM trees WHERE updated_by = ? AND updated_at >= ? AND LOWER(species) <> 'unknown' GROUP BY species ORDER BY use_count DESC LIMIT 10",
                &[Value::from(user_id), Value::from(since)],
            )
            .await?;

        let mut values: Vec<String> = Vec::new();

        for row in rows {
            values.push(row.require_string("species")?);
        }

        Ok(values)
    }

    pub async fn get_raw_stats(&self) -> Result<Vec<(String, u64)>> {
        let rows = self
            .db
            .fetch_sql(
                "SELECT species, COUNT(1) AS cnt FROM trees WHERE state <> 'gone' AND state <> 'stump' GROUP BY TRIM(LOWER(species)) ORDER BY cnt DESC, LOWER(species)",
                &[],
            )
            .await?;

        let mut res = Vec::new();

        for row in rows {
            let species = row.require_string("species")?;
            let count = row.require_u64("cnt")?;
            res.push((species, count));
        }

        Ok(res)
    }

    pub async fn get_counts(&self) -> Result<HashMap<String, u64>> {
        let rows = self
            .db
            .fetch_sql("SELECT species, COUNT(1) FROM trees GROUP BY species", &[])
            .await?;

        let mut counts = HashMap::new();

        for row in rows {
            let species = row.require_string("species")?;
            let count = row.require_u64("COUNT(1)")?;
            counts.insert(species, count);
        }

        Ok(counts)
    }

    fn from_row(row: &Attributes) -> Result<Species> {
        Ok(Species {
            name: row.require_string("name")?,
            local: row.require_string("local")?,
            keywords: row.require_string("keywords")?,
            wikidata_id: row.require_string("wikidata_id")?,
        })
    }
}

impl Injectable for SpeciesRepository {
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

    async fn setup() -> Arc<SpeciesRepository> {
        if env_logger::try_init().is_err() {
            debug!("env_logger already initialized.");
        };

        let state = AppState::new()
            .await
            .expect("Error creating app state.")
            .session()
            .await
            .expect("Error creating session state.");

        Arc::new(
            state
                .build::<SpeciesRepository>()
                .expect("Error creating species repository."),
        )
    }

    fn species(name: &str, local: &str, keywords: &str, wikidata_id: &str) -> Species {
        Species {
            name: name.to_string(),
            local: local.to_string(),
            keywords: keywords.to_string(),
            wikidata_id: wikidata_id.to_string(),
        }
    }

    #[tokio::test]
    async fn test_add_and_get() {
        let repo = setup().await;

        repo.add(&species(
            "Quercus robur",
            "Pedunculate oak",
            "дуб;oak",
            "Q165145",
        ))
        .await
        .expect("Error adding a species.");

        let res = repo
            .get("Quercus robur")
            .await
            .expect("Error getting species.");
        assert!(res.is_some());
        assert_eq!(res.unwrap().local, "Pedunculate oak");
    }

    #[tokio::test]
    async fn test_update() {
        let repo = setup().await;

        repo.add(&species(
            "Quercus robur",
            "Pedunculate oak",
            "дуб;oak",
            "Q165145",
        ))
        .await
        .expect("Error adding a species.");

        repo.update(&species(
            "Quercus robur",
            "Common oak",
            "дуб;oak",
            "Q165146",
        ))
        .await
        .expect("Error updating species.");

        let res = repo
            .get("Quercus robur")
            .await
            .expect("Error getting species.")
            .unwrap();
        assert_eq!(res.local, "Common oak");
        assert_eq!(res.wikidata_id, "Q165146");
    }

    #[tokio::test]
    async fn test_list() {
        let repo = setup().await;

        repo.db
            .execute_batch(include_str!("../../infra/database/fixtures/species.sql"))
            .await
            .expect("Error adding species.");

        let res = repo.all().await.expect("Error listing species.");

        assert_eq!(res.len(), 2);
        assert_eq!("Quercus robur", res[0].name);
        assert_eq!("Salix", res[1].name);
    }

    #[tokio::test]
    async fn test_get_counts() {
        let repo = setup().await;

        repo.db
            .execute_batch(
                "DELETE FROM trees;
                 INSERT INTO trees (id, lat, lon, species, added_at, updated_at, updated_by, added_by) VALUES (1, 40.1, 44.1, 'Quercus robur', 1, 1, 1, 1);
                 INSERT INTO trees (id, lat, lon, species, added_at, updated_at, updated_by, added_by) VALUES (2, 40.2, 44.2, 'Quercus robur', 1, 1, 1, 1);
                 INSERT INTO trees (id, lat, lon, species, added_at, updated_at, updated_by, added_by) VALUES (3, 40.3, 44.3, 'Salix', 1, 1, 1, 1);",
            )
            .await
            .expect("Error adding trees.");

        let counts = repo.get_counts().await.expect("Error getting counts.");

        assert_eq!(counts.len(), 2);
        assert_eq!(counts.get("Quercus robur"), Some(&2));
        assert_eq!(counts.get("Salix"), Some(&1));
    }
}
