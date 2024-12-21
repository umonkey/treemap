/**
 * The tree service performs updates to the trees database.
 *
 * Reading is straight-forward, updates also log individual property changes
 * to be able to track changes over time.
 */
use log::debug;
use std::sync::Arc;

use crate::services::Database;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp};

pub struct Trees {
    db: Arc<dyn Database>,
}

impl Trees {
    pub async fn new(db: &Arc<dyn Database>) -> Self {
        Self { db: db.clone() }
    }

    pub async fn update_tree(&self, req: UpdateTreeRequest) -> Result<TreeRecord> {
        let now = get_timestamp();

        let old = self.get_tree(req.id).await?;

        let new = TreeRecord {
            id: req.id,
            osm_id: old.osm_id,
            lat: old.lat,
            lon: old.lon,
            species: req.species.unwrap_or(old.species),
            notes: match req.notes {
                Some(value) => Some(value),
                None => old.notes,
            },
            height: match req.height {
                Some(value) => Some(value),
                None => old.height,
            },
            circumference: match fix_circumference(req.circumference) {
                Some(value) => Some(value),
                None => old.circumference,
            },
            diameter: match req.diameter {
                Some(value) => Some(value),
                None => old.diameter,
            },
            state: match req.state {
                Some(value) => value,
                None => old.state,
            },
            added_at: old.added_at,
            updated_at: now,
            added_by: old.added_by,
            thumbnail_id: old.thumbnail_id,
            year: match req.year {
                Some(value) => Some(value),
                None => old.year,
            },
        };

        debug!("Updating tree: {:?}", new);

        self.db.update_tree(&new).await?;

        Ok(new)
    }

    pub async fn move_tree(&self, req: &MoveTreeRequest) -> Result<()> {
        let id = req.id;
        let lat = req.lat;
        let lon = req.lon;

        debug!("Moving tree {} to ({}, {}).", id, lat, lon);

        self.db.move_tree(id, lat, lon).await?;

        Ok(())
    }

    pub async fn get_trees(&self, request: &GetTreesRequest) -> Result<TreeList> {
        let mut trees = self.db.get_trees(request.bounds()).await?;

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            trees.retain(|t| query.r#match(t));
        }

        Ok(TreeList::from_trees(trees))
    }

    pub async fn get_tree(&self, id: u64) -> Result<TreeRecord> {
        debug!("Getting details for tree {}.", id);

        match self.db.get_tree(id).await? {
            Some(tree) => Ok(tree),

            None => Err(Error::TreeNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::database::SqliteDatabase;
    use env_logger;
    use std::env;

    async fn setup(data: Option<String>) -> Result<Trees> {
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        let db = SqliteDatabase::new().await?;
        db.execute(include_str!("./fixtures/init.sql").to_string())
            .await?;

        if let Some(init_script) = data {
            db.execute(init_script).await?;
        }

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let dbh: Arc<dyn Database> = Arc::new(db);
        Ok(Trees::new(&dbh).await)
    }

    #[tokio::test]
    async fn test_get_no_trees() -> Result<()> {
        let service = setup(None).await?;

        let trees = service
            .get_trees(&GetTreesRequest {
                n: 90.0,
                e: 180.0,
                s: -90.0,
                w: -180.0,
                search: None,
            })
            .await?;

        assert_eq!(trees.trees.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_some_trees() -> Result<()> {
        let script = include_str!("./fixtures/some-trees.sql");

        let service = setup(Some(script.to_string()))
            .await
            .expect("Error initializing database");

        let trees = service
            .get_trees(&GetTreesRequest {
                n: 90.0,
                e: 180.0,
                s: -90.0,
                w: -180.0,
                search: None,
            })
            .await
            .expect("Error getting trees");

        assert_eq!(trees.trees.len(), 3);

        Ok(())
    }
}
