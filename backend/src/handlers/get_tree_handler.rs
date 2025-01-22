use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct GetTreeHandler {
    db: Arc<dyn DatabaseInterface>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
}

impl GetTreeHandler {
    pub async fn handle(&self, id: u64) -> Result<SingleTreeResponse> {
        let tree = match self.trees.get(id).await? {
            Some(value) => value,
            None => return Err(Error::TreeNotFound),
        };

        let files: Vec<FileRecord> = self
            .db
            .find_files_by_tree(id)
            .await?
            .into_iter()
            .filter(|file| file.deleted_at.is_none())
            .collect();

        let user_ids = self.collect_user_ids(&tree, &files).await?;
        let users = self.users.get_multiple(&user_ids).await?;

        Ok(SingleTreeResponse::from_tree(&tree, &files, &users))
    }

    async fn collect_user_ids(&self, tree: &TreeRecord, files: &[FileRecord]) -> Result<Vec<u64>> {
        let mut user_ids = Vec::new();

        user_ids.push(tree.added_by);

        for file in files {
            user_ids.push(file.added_by);
        }

        Ok(user_ids)
    }
}

impl Locatable for GetTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            db: locator.get::<PreferredDatabase>()?.driver(),
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
        })
    }
}
