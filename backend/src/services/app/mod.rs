use crate::objects::{Bounds, TreeList};
use crate::services::trees::Trees;
use crate::services::database::get_database;
use crate::Result;

pub struct AppState {
    trees: Trees,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let db = get_database().await?;

        Ok(Self {
            trees: Trees::init(&db).await,
        })
    }

    pub async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        self.trees.get_trees(bounds).await
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        self.trees.get_tree(id).await
    }
}
