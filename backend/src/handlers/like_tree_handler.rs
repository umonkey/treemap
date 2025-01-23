use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::get_timestamp;
use std::sync::Arc;

pub struct LikeTreeHandler {
    likes: Arc<LikeRepository>,
}

impl LikeTreeHandler {
    pub async fn handle(&self, tree_id: u64, user_id: u64) -> Result<()> {
        self.likes
            .add(&LikeRecord {
                tree_id,
                user_id,
                state: true,
                updated_at: get_timestamp(),
            })
            .await?;

        Ok(())
    }
}

impl Locatable for LikeTreeHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            likes: locator.get::<LikeRepository>()?,
        })
    }
}
