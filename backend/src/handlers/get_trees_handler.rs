use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreesHandler {
    loader: Arc<TreeListLoader>,
    trees: Arc<TreeRepository>,
}

impl GetTreesHandler {
    pub async fn handle(&self, request: &GetTreesRequest, user_id: u64) -> Result<TreeList> {
        let mut trees = self.trees.get_by_bounds(request.bounds()).await?;

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            trees.retain(|t| query.r#match(t, user_id));
        } else {
            trees.retain(Self::is_visible);
        }

        self.loader.load(&trees).await
    }

    fn is_visible(tree: &TreeRecord) -> bool {
        if tree.state == "gone" {
            return false;
        }

        if tree.species.to_lowercase().contains("error") {
            return false;
        }

        true
    }
}

impl Locatable for GetTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            loader: locator.get::<TreeListLoader>()?,
            trees: locator.get::<TreeRepository>()?,
        })
    }
}
