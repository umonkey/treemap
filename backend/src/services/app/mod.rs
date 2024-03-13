use crate::objects::TreeList;
use crate::services::trees::Trees;
use crate::services::SqliteDatabase;
use crate::Result;

pub struct AppState {
    trees: Trees,
}

impl AppState {
    pub async fn init() -> Result<Self> {
        let db = SqliteDatabase::init().await?;

        Ok(Self {
            trees: Trees::init(&db).await,
        })
    }

    pub async fn get_trees(&self) -> Result<TreeList> {
        self.trees.get_trees().await
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        self.trees.get_tree(id).await
    }
}
