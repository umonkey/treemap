//! Returns a list of all changes made to this tree.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeHistoryHandler {
    props: Arc<PropRepository>,
    users: Arc<UserRepository>,
}

impl GetTreeHistoryHandler {
    pub async fn handle(&self, tree_id: u64) -> Result<PropList> {
        let props = self.props.find_by_tree(tree_id).await?;

        let user_ids: Vec<u64> = props.iter().map(|r| r.added_by).collect();
        let users = self.users.get_multiple(&user_ids).await?;

        Ok(PropList::from_records(&props, &users))
    }
}

impl Locatable for GetTreeHistoryHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            props: locator.get::<PropRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
