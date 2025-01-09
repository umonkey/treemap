use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeDefaultsHandler {
    trees: Arc<TreeRepository>,
}

impl GetTreeDefaultsHandler {
    pub async fn handle(&self, user_id: u64) -> Result<NewTreeDefaultsResponse> {
        match self.trees.get_last_by_user(user_id).await? {
            Some(tree) => Ok(NewTreeDefaultsResponse::from_tree(&tree)),

            None => Ok(NewTreeDefaultsResponse {
                species: None,
                notes: None,
                height: Some(0.0),
                circumference: Some(0.0),
                diameter: Some(0.0),
                state: Some("healthy".to_string()),
            }),
        }
    }
}

impl Locatable for GetTreeDefaultsHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
