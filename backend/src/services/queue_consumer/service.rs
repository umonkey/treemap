use super::schemas::*;
use crate::domain::photo::PhotoService;
use crate::domain::tree::TreeService;
use crate::domain::user::*;
use crate::infra::queue::{Queue, QueueMessage};
use crate::services::*;
use crate::types::*;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;

// Seconds to wait for a new message.
const DELAY: u64 = 1;

pub struct QueueConsumer {
    queue: Arc<Queue>,
    trees: Arc<TreeService>,
    photos: Arc<PhotoService>,
    users: Arc<UserService>,
}

impl QueueConsumer {
    pub async fn run(&self) {
        info!("Running queue consumer.");

        loop {
            let msg = self.queue.pop().await;

            match msg {
                Ok(Some(msg)) => {
                    match self.process_message(msg.payload.as_str()).await {
                        Ok(_) => self.delete_message(&msg).await,
                        Err(e) => self.delay_message(&e, &msg).await,
                    };
                }

                Ok(None) => {
                    tokio::time::sleep(Duration::from_secs(DELAY)).await;
                }

                Err(e) => {
                    error!("Error while processing queue message: {e:?}");
                }
            }
        }
    }

    async fn delete_message(&self, msg: &QueueMessage) {
        if let Err(e) = self.queue.delete(msg).await {
            error!("Error deleting message from queue: {e}");
        }
    }

    async fn delay_message(&self, e: &Error, msg: &QueueMessage) {
        error!("Message failed: {e}, delaying.");

        if let Err(e) = self.queue.delay(msg).await {
            error!("Error delaying message: {e}");
        }
    }

    /**
     * Decode the message from JSON and process it.
     */
    async fn process_message(&self, msg: &str) -> Result<()> {
        match QueueCommand::decode(msg) {
            Ok(Some(QueueCommand::ResizeImage(message))) => {
                self.photos.resize_image(message.id).await?;
            }

            Ok(Some(QueueCommand::UpdateTreeAddress(message))) => {
                self.trees.update_tree_address(message.id).await?;
            }

            Ok(Some(QueueCommand::AddPhoto(message))) => {
                self.photos
                    .add_tree_photo(message.tree_id, message.file_id)
                    .await?;
            }

            Ok(Some(QueueCommand::UpdateUserpic(message))) => {
                self.users
                    .update_userpic(message.user_id, message.file_id)
                    .await?;
            }

            Ok(None) => {
                debug!("Unknown message: {msg}");
            }

            Err(e) => {
                error!("Error while decoding message: {e:?}");
            }
        }

        Ok(())
    }
}

impl Locatable for QueueConsumer {
    fn create(locator: &Locator) -> Result<Self> {
        Ok(Self {
            queue: locator.get::<Queue>()?,
            trees: locator.get::<TreeService>()?,
            photos: locator.get::<PhotoService>()?,
            users: locator.get::<UserService>()?,
        })
    }
}
