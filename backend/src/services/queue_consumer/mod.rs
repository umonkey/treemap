/// This code reads messages from the queue and processes them,
/// by decoding parameters and calling the appropriate service.
///
/// Runs in an infinite loop.
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
}

impl QueueConsumer {
    pub async fn new() -> Result<Self> {
        let locator = Locator::new();

        Ok(Self {
            queue: locator.get::<QueueService>()?,
            resize_image_handler: locator.get::<ResizeImageHandler>()?,
        })
    }

    pub async fn run(&self) {
        info!("Running queue consumer.");

        loop {
            let msg = self.queue.pop().await;

            match msg {
                Ok(Some(msg)) => {
                    match self.process_message(msg.payload.as_str()).await {
                        Ok(_) => {
                            self.queue.delete(&msg).await.unwrap();
                        }

                        Err(e) => {
                            error!("Error while processing message: {:?}", e);
                        }
                    };
                }

                Ok(None) => {
                    tokio::time::sleep(Duration::from_secs(DELAY)).await;
                }

                Err(e) => {
                    error!("Error while processing queue message: {:?}", e);
                }
            }
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

            Ok(None) => {
                debug!("Unknown message: {}", msg);
            }

            Err(e) => {
                error!("Error while decoding message: {:?}", e);
            }
        }

        Ok(())
    }
}
