use super::schemas::*;
use crate::actions::file::FileRead;
use crate::domain::file::{file_url, FileRepository};
use crate::domain::tree::Tree;
use crate::domain::tree_file::TreeFileRepository;
use crate::domain::user::{User, UserRepository};
use crate::infra::config::Config;
use crate::services::*;
use crate::types::Result;
use crate::utils::unique_ids;
use log::debug;
use std::collections::HashSet;
use std::sync::Arc;

pub struct TreeLoader {
    users: Arc<UserRepository>,
    files: Arc<FileRepository>,
    tree_files: Arc<TreeFileRepository>,
    config: Arc<Config>,
}

impl TreeLoader {
    pub async fn load_list(&self, trees: &[Tree]) -> Result<TreeList> {
        debug!("Enriching a list of {} trees.", trees.len());

        let user_ids: Vec<u64> = trees.iter().map(|t| t.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        debug!("Found {} users.", users.len());

        let mut res = TreeList::from_trees(trees).with_users(&users);

        let base_url = self.base_url();

        for (read, tree) in res.trees.iter_mut().zip(trees.iter()) {
            read.thumbnail_url = thumbnail_url(&base_url, tree.thumbnail_file_id);
        }

        debug!("Finished building the list.");

        Ok(res)
    }

    pub async fn load_single(&self, tree: &Tree) -> Result<SingleTreeResponse> {
        let mut files = self.find_files(tree.id).await?;

        if let Some(thumbnail_file_id) = tree.thumbnail_file_id {
            if let Some(pos) = files
                .iter()
                .position(|file| file.id == thumbnail_file_id.to_string())
            {
                let thumbnail = files.remove(pos);
                files.insert(0, thumbnail);
            }
        }

        let user_ids = self.collect_user_ids(tree, &files).await?;
        let users = self.users.get_multiple(&user_ids).await?;

        let mut response = SingleTreeResponse::from_tree(tree, &files, &users);
        response.thumbnail_url = thumbnail_url(&self.base_url(), tree.thumbnail_file_id);

        Ok(response)
    }

    async fn load_users(&self, user_ids: &[u64]) -> Result<Vec<User>> {
        let ids = unique_ids(user_ids);
        let users = self.users.get_multiple(&ids).await?;
        Ok(users)
    }

    async fn collect_user_ids(&self, tree: &Tree, files: &[FileRead]) -> Result<Vec<u64>> {
        let mut user_ids = Vec::new();

        user_ids.push(tree.added_by);

        for file in files {
            if let Ok(user_id) = file.added_by.parse::<u64>() {
                user_ids.push(user_id);
            }
        }

        Ok(user_ids)
    }

    async fn find_files(&self, tree_id: u64) -> Result<Vec<FileRead>> {
        let base_url = self.base_url();
        let mut files = Vec::new();
        let mut seen = HashSet::new();

        for link in self.tree_files.find_by_tree(tree_id).await? {
            if link.is_deleted() {
                continue;
            }

            if !seen.insert(link.file_id) {
                continue;
            }

            if let Some(details) = self.files.get_details(link.file_id).await? {
                if details.is_listable() {
                    files.push(FileRead::from_file_details(&details, &base_url));
                }
            }
        }

        Ok(files)
    }

    fn base_url(&self) -> String {
        self.config.files_base_url.clone()
    }
}

impl Injectable for TreeLoader {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            users: Arc::new(ctx.build::<UserRepository>()?),
            files: Arc::new(ctx.build::<FileRepository>()?),
            tree_files: Arc::new(ctx.build::<TreeFileRepository>()?),
            config: ctx.config(),
        })
    }
}

fn thumbnail_url(base_url: &str, thumbnail_file_id: Option<u64>) -> Option<String> {
    thumbnail_file_id.map(|id| file_url(base_url, &format!("files/{id}/small.jpg")))
}
