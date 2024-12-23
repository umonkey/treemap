use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreesHandler {
    db: Arc<dyn DatabaseInterface>,
    loader: Arc<TreeListLoader>,
}

impl GetTreesHandler {
    pub async fn handle(&self, request: &GetTreesRequest) -> Result<TreeList> {
        let mut trees = self.db.get_trees(request.bounds()).await?;

        if let Some(search) = &request.search {
            let query = SearchQuery::from_string(search);
            trees.retain(|t| query.r#match(t));
        }

        self.loader.load(&trees).await
    }
}

impl Locatable for GetTreesHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let db = locator.get::<PreferredDatabase>()?.driver();
        let loader = locator.get::<TreeListLoader>()?;
        Ok(Self { db, loader })
    }
}
