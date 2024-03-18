use std::sync::Arc;
use log::debug;

use crate::Result;
use crate::errors::Error;
use crate::objects::{Bounds, TreeList};
use crate::services::Database;

pub struct Trees {
    db: Arc<dyn Database>,
}

impl Trees {
    pub async fn init(db: &Arc<dyn Database>) -> Self {
        Self {
            db: db.clone(),
        }
    }

    pub async fn get_trees(&self, bounds: Bounds) -> Result<TreeList> {
        self.db.get_trees(bounds).await
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        debug!("Getting details for tree {}.", id);

        Err(Error::TreeNotFound)
    }
}
