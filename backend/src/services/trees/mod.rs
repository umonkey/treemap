use log::debug;

use crate::Result;
use crate::errors::Error;
use crate::objects::TreeList;
use crate::services::SqliteDatabase;

pub struct Trees {
    db: SqliteDatabase,
}

impl Trees {
    pub async fn init(db: &SqliteDatabase) -> Self {
        Self {
            db: db.clone(),
        }
    }

    pub async fn get_trees(&self) -> Result<TreeList> {
        self.db.get_trees().await
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        debug!("Getting details for tree {}.", id);

        Err(Error::TreeNotFound)
    }
}
