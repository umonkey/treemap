use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreesHandler {
    db: Arc<dyn DatabaseInterface>,
}

impl GetTreesHandler {
    pub async fn handle(&self, request: &GetTreesRequest) -> Result<TreeList> {
        let mut trees = self.db.get_trees(request.bounds()).await?;

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            trees.retain(|t| query.r#match(t));
        }

        Ok(TreeList::from_trees(&trees))
    }
}

impl Locatable for GetTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        Ok(Self { db })
    }
}
