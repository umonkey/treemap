use crate::services::*;
use crate::types::*;
use std::sync::Arc;

pub struct AddCommentHandler {
    comments: Arc<CommentInjector>,
}

impl AddCommentHandler {
    pub async fn handle(&self, req: AddCommentRequest) -> Result<()> {
        self.comments
            .inject(req.tree_id, req.user_id, &req.message)
            .await?;

        Ok(())
    }
}

impl Locatable for AddCommentHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            comments: locator.get::<CommentInjector>()?,
        })
    }
}
