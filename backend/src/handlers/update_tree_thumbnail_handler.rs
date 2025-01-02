use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeThumbnailHandler {
    trees: Arc<TreeRepository>,
    files: Arc<FileRepository>,
    props: Arc<PropRepository>,
}

impl UpdateTreeThumbnailHandler {
    pub async fn handle(&self, req: UpdateTreeThumbnailRequest) -> Result<()> {
        let tree = self.trees.get(req.tree_id).await?;
        let file = self.files.get(req.file_id).await?;

        self.trees.update_thumbnail(tree.id, file.small_id).await?;

        self.props
            .add(&PropRecord {
                tree_id: tree.id,
                added_by: req.user_id,
                name: "thumbnail_id".to_string(),
                value: file.small_id.to_string(),
                ..Default::default()
            })
            .await?;

        info!(
            "Thumbnail for tree {} changed to {} by {}.",
            req.tree_id, req.file_id, req.user_id
        );

        Ok(())
    }
}

impl Locatable for UpdateTreeThumbnailHandler {
    fn create(locator: &Locator) -> Result<Self> {
        let trees = locator.get::<TreeRepository>()?;
        let files = locator.get::<FileRepository>()?;
        let props = locator.get::<PropRepository>()?;
        Ok(Self {
            trees,
            files,
            props,
        })
    }
}
