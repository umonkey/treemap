use crate::objects::TreeList;
use crate::services::trees::Trees;
use crate::Result;

pub struct AppState {
    trees: Trees,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        Ok(Self {
            trees: Trees::init().await,
        })
    }

    pub async fn get_trees(&self) -> Result<TreeList> {
        self.trees.get_trees().await
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        self.trees.get_tree(id).await
    }
}
