//! Add a story (message handler).
//!
//! This message is sent whenever a user adds a photo to a tree.
//! We then make sure that the user doesn't have any recent stories
//! about this tree, and add a new one.

use crate::common::database::repositories::*;
use crate::services::*;
use crate::types::*;
use crate::utils::*;
use log::{debug, error, info};
use std::sync::Arc;

pub struct AddStoryHandler {
    stories: Arc<StoryRepository>,
}

impl AddStoryHandler {
    pub async fn handle(&self, msg: AddStoryMessage) -> Result<()> {
        if self.find_existing(&msg).await? {
            return Ok(());
        }

        let story = StoryRecord {
            id: get_unique_id()?,
            user_id: msg.user_id,
            tree_id: msg.tree_id,
            file_id: msg.file_id,
            added_at: msg.added_at,
            text: msg.text.clone(),
        };

        self.stories.add(story).await.inspect_err(|e| {
            error!("Could not add story: {:?}", e);
        })?;

        info!(
            "Story for user {} on tree {} added successfully.",
            msg.user_id, msg.tree_id
        );

        Ok(())
    }

    async fn find_existing(&self, msg: &AddStoryMessage) -> Result<bool> {
        let stories = self.stories.find_by_user(msg.user_id).await?;
        let after = get_timestamp() - 24 * 60 * 60; // 24 hours ago

        for story in stories {
            if story.tree_id == msg.tree_id && story.added_at > after {
                debug!(
                    "Found existing story for user {} on tree {}: {:?}",
                    msg.user_id, msg.tree_id, story
                );
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl Locatable for AddStoryHandler {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            stories: locator.get::<StoryRepository>()?,
        })
    }
}
