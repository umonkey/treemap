use super::models::PropRecord;
use super::repository::PropRepository;
use crate::services::{Locatable, Locator};
use crate::types::Result;
use std::sync::Arc;

pub struct PropService {
    props: Arc<PropRepository>,
}

impl PropService {
    pub async fn get_history(&self, tree_id: u64) -> Result<Vec<PropRecord>> {
        self.props.find_by_tree(tree_id).await
    }
}

impl Locatable for PropService {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            props: locator.get::<PropRepository>()?,
        })
    }
}
