use crate::common::database::repositories::*;
use crate::infra::queue::Queue;
use crate::services::*;
use crate::types::*;
use crate::utils::{fix_circumference, get_timestamp};
use log::info;
use std::sync::Arc;

pub struct UpdateTreeHandler {
    files: Arc<FileRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    queue: Arc<Queue>,
}

impl UpdateTreeHandler {
    pub async fn handle(&self, req: UpdateTreeRequest) -> Result<SingleTreeResponse> {
        let now = get_timestamp();

        let old = match self.trees.get(req.id).await? {
            Some(tree) => tree,
            None => return Err(Error::TreeNotFound),
        };

        let new = TreeRecord {
            id: req.id,
            osm_id: old.osm_id,
            species: req.species.unwrap_or(old.species),
            lat: req.lat.unwrap_or(old.lat),
            lon: req.lon.unwrap_or(old.lon),
            notes: match req.notes {
                Some(value) => Some(value),
                None => old.notes,
            },
            height: match req.height {
                Some(value) => Some(value),
                None => old.height,
            },
            circumference: match fix_circumference(req.circumference) {
                Some(value) => Some(value),
                None => old.circumference,
            },
            diameter: match req.diameter {
                Some(value) => Some(value),
                None => old.diameter,
            },
            state: match req.state {
                Some(value) => value,
                None => old.state,
            },
            added_at: old.added_at,
            updated_at: now,
            added_by: old.added_by,
            thumbnail_id: old.thumbnail_id,
            year: match req.year {
                Some(value) => Some(value),
                None => old.year,
            },
            address: match req.address {
                Some(value) => Some(value),
                None => old.address,
            },
            ..old
        };

        info!("Updating tree: {new:?}");

        self.trees.update(&new, req.user_id).await?;

        if new.address.is_none() {
            self.schedule_address_update(new.id).await?;
        }

        self.users.increment_update_count(req.user_id).await?;

        let files = self.find_files(new.id).await?;
        let users = self.find_users(&new, &files).await?;

        Ok(SingleTreeResponse::from_tree(&new, &files, &users))
    }

    async fn find_files(&self, tree_id: u64) -> Result<Vec<FileRecord>> {
        let files = self.files.find_by_tree(tree_id).await?;
        Ok(files.into_iter().filter(|file| file.is_visible()).collect())
    }

    async fn find_users(&self, tree: &TreeRecord, files: &[FileRecord]) -> Result<Vec<UserRecord>> {
        let mut user_ids = Vec::new();

        user_ids.push(tree.added_by);

        for file in files {
            user_ids.push(file.added_by);
        }

        let users = self.users.get_multiple(&user_ids).await?;

        Ok(users)
    }

    async fn schedule_address_update(&self, tree_id: u64) -> Result<()> {
        let msg = UpdateTreeAddressMessage { id: tree_id };
        self.queue.push(&msg.encode()).await?;

        info!("Scheduled address update for tree {tree_id}");

        Ok(())
    }
}

impl Locatable for UpdateTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            files: locator.get::<FileRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            queue: locator.get::<Queue>()?,
        })
    }
}
