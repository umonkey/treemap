use crate::objects::{AddTreeRequest, Bounds, TreeInfo, TreeList};
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

    pub async fn add_tree(&self, req: AddTreeRequest) -> Result<TreeInfo> {
        self.trees.add_tree(req).await
    }

    pub async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        self.trees.get_trees(bounds).await
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        self.trees.get_tree(id).await
    }
}
