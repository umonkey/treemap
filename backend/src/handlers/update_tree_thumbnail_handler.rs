use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeThumbnailHandler {
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    files: Arc<FileRepository>,
}

impl UpdateTreeThumbnailHandler {
    pub async fn handle(&self, req: UpdateTreeThumbnailRequest) -> Result<()> {
        let tree = self
            .trees
            .get(req.tree_id)
            .await?
            .ok_or(Error::TreeNotFound)?;

        let file = self
            .files
            .get(req.file_id)
            .await?
            .ok_or(Error::FileNotFound)?;

        self.trees
            .update_thumbnail(tree.id, file.small_id, req.user_id)
            .await?;

        self.users.increment_update_count(req.user_id).await?;

        info!(
            "Thumbnail for tree {} changed to {} by {}.",
            req.tree_id, req.file_id, req.user_id
        );

        Ok(())
    }
}

impl Locatable for UpdateTreeThumbnailHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            files: locator.get::<FileRepository>()?,
        })
    }
}
