use log::debug;

use crate::Result;
use crate::errors::Error;
use crate::objects::{TreeInfo, TreeList};

pub struct Trees {
}

impl Trees {
    pub async fn init() -> Self {
        Self { }
    }

    pub async fn get_trees(&self) -> Result<TreeList> {
        let mut trees: Vec<TreeInfo> = Vec::new();

        let tree = TreeInfo::create(1, 1.2, 3.4);
        trees.push(tree);

        let tree_list = TreeList::create(trees);

        Ok(tree_list)
    }

    pub async fn get_tree(&self, id: u64) -> Result<()> {
        debug!("Getting details for tree {}.", id);

        Err(Error::TreeNotFound)
    }
}
