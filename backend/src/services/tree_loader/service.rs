use super::schemas::*;
use crate::domain::tree::Tree;
use crate::domain::tree_image::{TreeImage, TreeImageRepository};
use crate::domain::user::{User, UserRepository};
use crate::services::*;
use crate::types::Result;
use crate::utils::unique_ids;
use log::debug;
use std::sync::Arc;

pub struct TreeLoader {
    users: Arc<UserRepository>,
    files: Arc<TreeImageRepository>,
}

impl TreeLoader {
    pub async fn load_list(&self, trees: &[Tree]) -> Result<TreeList> {
        debug!("Enriching a list of {} trees.", trees.len());

        let user_ids: Vec<u64> = trees.iter().map(|t| t.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        debug!("Found {} users.", users.len());

        let res = TreeList::from_trees(trees).with_users(&users);

        debug!("Finished building the list.");

        Ok(res)
    }

    pub async fn load_single(&self, tree: &Tree) -> Result<SingleTreeResponse> {
        let files = self.find_files(tree.id).await?;

        let user_ids = self.collect_user_ids(tree, &files).await?;
        let users = self.users.get_multiple(&user_ids).await?;

        Ok(SingleTreeResponse::from_tree(tree, &files, &users))
    }

    async fn load_users(&self, user_ids: &[u64]) -> Result<Vec<User>> {
        let ids = unique_ids(user_ids);
        let users = self.users.get_multiple(&ids).await?;
        Ok(users)
    }

    async fn collect_user_ids(&self, tree: &Tree, files: &[TreeImage]) -> Result<Vec<u64>> {
        let mut user_ids = Vec::new();

        user_ids.push(tree.added_by);

        for file in files {
            user_ids.push(file.added_by);
        }

        Ok(user_ids)
    }

    async fn find_files(&self, tree_id: u64) -> Result<Vec<TreeImage>> {
        let files = self.files.find_by_tree(tree_id).await?;
        Ok(files.into_iter().filter(|file| file.is_visible()).collect())
    }
}

impl Locatable for TreeLoader {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            users: locator.get::<UserRepository>()?,
            files: locator.get::<TreeImageRepository>()?,
        })
    }
}
