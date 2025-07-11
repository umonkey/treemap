//! This code reads messages from the queue and processes them,
//! by decoding parameters and calling the appropriate service.
//!
//! Runs in an infinite loop.

use crate::handlers::*;
use crate::services::*;
use crate::types::*;
use log::{debug, error, info};
use std::sync::Arc;
use std::time::Duration;

// Seconds to wait for a new message.
const DELAY: u64 = 1;

pub struct QueueConsumer {
    queue: Arc<QueueService>,
    resize_image_handler: Arc<ResizeImageHandler>,
    update_tree_address_handler: Arc<UpdateTreeAddressHandler>,
    add_photo_handler: Arc<AddPhotoHandler>,
    update_userpic_handler: Arc<UpdateUserpicHandler>,
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
                        Err(e) => error!("Error while processing message: {e:?}"),
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

    /**
     * Decode the message from JSON and process it.
     */
    async fn process_message(&self, msg: &str) -> Result<()> {
        match QueueCommand::decode(msg) {
            Ok(Some(QueueCommand::ResizeImage(message))) => {
                self.resize_image_handler.handle(message.id).await?;
            }

            Ok(Some(QueueCommand::UpdateTreeAddress(message))) => {
                self.update_tree_address_handler.handle(message.id).await?;
            }

            Ok(Some(QueueCommand::AddPhoto(message))) => {
                self.add_photo_handler
                    .handle(message.tree_id, message.file_id)
                    .await?;
            }

            Ok(Some(QueueCommand::UpdateUserpic(message))) => {
                self.update_userpic_handler
                    .handle(message.user_id, message.file_id)
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
            queue: locator.get::<QueueService>()?,
            resize_image_handler: locator.get::<ResizeImageHandler>()?,
            update_tree_address_handler: locator.get::<UpdateTreeAddressHandler>()?,
            add_photo_handler: locator.get::<AddPhotoHandler>()?,
            update_userpic_handler: locator.get::<UpdateUserpicHandler>()?,
        })
    }
}
