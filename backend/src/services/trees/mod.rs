use std::sync::Arc;
use log::debug;

use crate::Result;
use crate::errors::Error;
use crate::services::Database;
use crate::types::{AddTreeRequest, Bounds, TreeInfo, TreeList, TreeListItem};
use crate::utils::{get_unique_id, get_timestamp};

pub struct Trees {
    db: Arc<dyn Database>,
}

impl Trees {
    pub async fn init(db: &Arc<dyn Database>) -> Self {
        Self {
            db: db.clone(),
        }
    }

    pub async fn add_tree(&self, req: AddTreeRequest) -> Result<TreeInfo> {
        let id = get_unique_id()?;
        let now = get_timestamp();

        let tree = TreeInfo {
            id,
            lat: req.lat,
            lon: req.lon,
            name: req.name,
            height: req.height,
            circumference: req.circumference,
            added_at: now,
            updated_at: now,
            added_by: 0,
        };

        debug!("Adding tree at ({}, {}) with name '{}'.", req.lat, req.lon, tree.name);

        self.db.add_tree(&tree).await?;
        self.db.add_tree_prop(tree.id, "lat", &tree.lat.to_string()).await?;
        self.db.add_tree_prop(tree.id, "lon", &tree.lon.to_string()).await?;
        self.db.add_tree_prop(tree.id, "name", &tree.name.to_string()).await?;

        if let Some(height) = tree.height {
            self.db.add_tree_prop(tree.id, "height", &height.to_string()).await?;
        }

        if let Some(circumference) = tree.circumference {
            self.db.add_tree_prop(tree.id, "circumference", &circumference.to_string()).await?;
        }

        Ok(tree)
    }

    pub async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        let trees = self.db.get_trees(bounds).await?;

        let items = trees.iter().map(|tree| {
            TreeListItem::from_tree(tree)
        }).collect();

        Ok(TreeList {
            trees: items,
        })
    }

    pub async fn get_tree(&self, id: u64) -> Result<TreeInfo> {
        debug!("Getting details for tree {}.", id);

        match self.db.get_tree(id).await? {
            Some(tree) => {
                Ok(tree)
            },

            None => Err(Error::TreeNotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use env_logger;
    use crate::services::database::SqliteDatabase;

    async fn setup(data: Option<String>) -> Result<Trees> {
        env::set_var("TREEMAP_SQLITE_PATH", ":memory:");

        let db = SqliteDatabase::new().await?;
        db.execute(include_str!("./fixtures/init.sql").to_string()).await?;

        if let Some(init_script) = data {
            db.execute(init_script).await?;
        }

        if let Err(_) = env_logger::try_init() {
            debug!("env_logger already initialized.");
        };

        let dbh: Arc<dyn Database> = Arc::new(db);
        Ok(Trees::init(&dbh).await)
    }

    #[tokio::test]
    async fn test_get_no_trees() -> Result<()> {
        let service = setup(None).await?;

        let trees = service.get_trees(Bounds {
            n: 90.0,
            e: 180.0,
            s: -90.0,
            w: -180.0,
        }).await?;

        assert_eq!(trees.trees.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_get_some_trees() -> Result<()> {
        let script = include_str!("./fixtures/some-trees.sql");
        let service = setup(Some(script.to_string())).await?;

        let trees = service.get_trees(Bounds {
            n: 90.0,
            e: 180.0,
            s: -90.0,
            w: -180.0,
        }).await?;

        assert_eq!(trees.trees.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_add_tree() -> Result<()> {
        let service = setup(None).await?;

        let tree = service.add_tree(AddTreeRequest {
            lat: 12.34,
            lon: 56.78,
            name: "Oak".to_string(),
            height: Some(10.0),
            circumference: Some(20.0),
        }).await?;

        debug!("Tree added: {:?}", tree);

        assert_eq!(tree.lat, 12.34);
        assert_eq!(tree.lon, 56.78);

        Ok(())
    }
}
