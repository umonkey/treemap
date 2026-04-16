use super::models::PropRecord;
use super::repository::PropRepository;
use crate::services::{Context, Injectable};
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

impl Injectable for PropService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            props: Arc::new(ctx.build::<PropRepository>()?),
        })
    }
}
