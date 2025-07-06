use crate::common::database::repositories::*;
use crate::handlers::GetTreeHandler;
use crate::services::*;
use crate::types::*;
use log::info;
use std::sync::Arc;

pub struct UpdateTreeStateHandler {
    props: Arc<PropRepository>,
    trees: Arc<TreeRepository>,
    users: Arc<UserRepository>,
    getter: Arc<GetTreeHandler>,
    comments: Arc<CommentInjector>,
}

impl UpdateTreeStateHandler {
    pub async fn handle(
        &self,
        tree_id: u64,
        value: String,
        user_id: u64,
        comment: Option<String>,
    ) -> Result<SingleTreeResponse> {
        // Validate that tree exists.
        let tree = self.trees.get(tree_id).await?.ok_or(Error::TreeNotFound)?;

        // The update method only logs changes, but for this explicit handler
        // we need to save all data, even if no changes were made (confirm the value).
        if tree.state == value {
            self.props
                .add(&PropRecord {
                    tree_id,
                    name: "state".to_string(),
                    value: value.clone(),
                    added_by: user_id,
                    ..Default::default()
                })
                .await?;
        }

        self.trees
            .update(
                &TreeRecord {
                    state: value.clone(),
                    ..tree
                },
                user_id,
            )
            .await?;

        self.users.increment_update_count(user_id).await?;

        // Add comment if provided
        if let Some(comment_text) = comment {
            if !comment_text.trim().is_empty() {
                self.comments
                    .inject(tree_id, user_id, &comment_text)
                    .await?;
            }
        }

        info!("State for tree {tree_id} changed to {value} by {user_id}.");

        self.getter.handle(tree_id).await
    }
}

impl Locatable for UpdateTreeStateHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            props: locator.get::<PropRepository>()?,
            trees: locator.get::<TreeRepository>()?,
            users: locator.get::<UserRepository>()?,
            getter: locator.get::<GetTreeHandler>()?,
            comments: locator.get::<CommentInjector>()?,
        })
    }
}
