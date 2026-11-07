use super::schemas::*;
use crate::actions::file::FileRead;
use crate::domain::file::{file_url, FileRepository};
use crate::domain::tree::Tree;
use crate::domain::tree_file::TreeFileRepository;
use crate::domain::tree_image::TreeImageRepository;
use crate::domain::user::{User, UserRepository};
use crate::infra::config::Config;
use crate::infra::database::{Database, Value};
use crate::services::*;
use crate::types::Result;
use crate::utils::unique_ids;
use log::debug;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub struct TreeLoader {
    db: Arc<Database>,
    users: Arc<UserRepository>,
    files: Arc<FileRepository>,
    tree_files: Arc<TreeFileRepository>,
    tree_images: Arc<TreeImageRepository>,
    config: Arc<Config>,
}

impl TreeLoader {
    pub async fn load_list(&self, trees: &[Tree]) -> Result<TreeList> {
        debug!("Enriching a list of {} trees.", trees.len());

        let user_ids: Vec<u64> = trees.iter().map(|t| t.added_by).collect();
        let users = self.load_users(&user_ids).await?;

        debug!("Found {} users.", users.len());

        let mut res = TreeList::from_trees(trees).with_users(&users);

        let tree_ids: Vec<u64> = trees.iter().map(|tree| tree.id).collect();
        let thumbnail_ids = self.load_thumbnail_file_ids(&tree_ids).await?;
        let base_url = self.base_url();

        for (read, tree) in res.trees.iter_mut().zip(trees.iter()) {
            let thumbnail_file_id = thumbnail_ids.get(&tree.id).copied().flatten();
            read.thumbnail_url = thumbnail_url(&base_url, thumbnail_file_id, tree.thumbnail_id);
        }

        debug!("Finished building the list.");

        Ok(res)
    }

    pub async fn load_single(&self, tree: &Tree) -> Result<SingleTreeResponse> {
        let mut files = self.find_files(tree.id).await?;

        let thumbnail_ids = self.load_thumbnail_file_ids(&[tree.id]).await?;
        let thumbnail_file_id = thumbnail_ids.get(&tree.id).copied().flatten();

        if let Some(thumbnail_file_id) = thumbnail_file_id {
            if let Some(pos) = files
                .iter()
                .position(|file| file.id == thumbnail_file_id.to_string())
            {
                let thumbnail = files.remove(pos);
                files.insert(0, thumbnail);
            }
        } else if let Some(thumbnail_id) = tree.thumbnail_id {
            if let Some(pos) = files
                .iter()
                .position(|file| file.small_id == thumbnail_id.to_string())
            {
                let thumbnail = files.remove(pos);
                files.insert(0, thumbnail);
            }
        }

        let user_ids = self.collect_user_ids(tree, &files).await?;
        let users = self.users.get_multiple(&user_ids).await?;

        let mut response = SingleTreeResponse::from_tree(tree, &files, &users);
        response.thumbnail_url =
            thumbnail_url(&self.base_url(), thumbnail_file_id, tree.thumbnail_id);

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

        for image in self.tree_images.find_by_tree(tree_id).await? {
            if !image.is_visible() {
                continue;
            }

            let file_id = if image.source_id > 0 {
                image.source_id
            } else {
                image.id
            };

            if !seen.insert(file_id) {
                continue;
            }

            files.push(FileRead::from_tree_image(&image, &base_url));
        }

        Ok(files)
    }

    async fn load_thumbnail_file_ids(&self, tree_ids: &[u64]) -> Result<HashMap<u64, Option<u64>>> {
        if tree_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let placeholders = vec!["?"; tree_ids.len()].join(", ");
        let sql =
            format!("SELECT `id`, `thumbnail_file_id` FROM `trees` WHERE `id` IN ({placeholders})");

        let params: Vec<Value> = tree_ids.iter().map(|id| Value::from(*id)).collect();
        let rows = self.db.fetch_sql(&sql, &params).await?;

        let mut result = HashMap::new();

        for row in rows {
            let id = row.require_u64("id")?;
            let thumbnail_file_id = row.get_u64("thumbnail_file_id")?;
            result.insert(id, thumbnail_file_id);
        }

        Ok(result)
    }

    fn base_url(&self) -> String {
        self.config.files_base_url.clone()
    }
}

impl Injectable for TreeLoader {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            users: Arc::new(ctx.build::<UserRepository>()?),
            files: Arc::new(ctx.build::<FileRepository>()?),
            tree_files: Arc::new(ctx.build::<TreeFileRepository>()?),
            tree_images: Arc::new(ctx.build::<TreeImageRepository>()?),
            config: ctx.config(),
        })
    }
}

fn thumbnail_url(
    base_url: &str,
    thumbnail_file_id: Option<u64>,
    thumbnail_id: Option<u64>,
) -> Option<String> {
    if let Some(file_id) = thumbnail_file_id {
        return Some(file_url(base_url, &format!("files/{file_id}/small.jpg")));
    }

    thumbnail_id.map(|id| file_url(base_url, &id.to_string()))
}
