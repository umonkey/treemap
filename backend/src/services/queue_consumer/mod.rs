/**
 * This code reads messages from the queue and processes them,
 * by decoding parameters and calling the appropriate service.
 *
 * Runs in an infinite loop.
 */
use log::{debug, error, info};
use std::time::Duration;
use std::sync::Arc;

use crate::services::database::get_database;
use crate::services::{FileService, QueueService, S3Service};
use crate::types::{QueueCommand, Result};

// Seconds to wait for a new message.
const DELAY: u64 = 1;

pub struct QueueConsumer {
    files: FileService,
    queue: QueueService,
}

impl QueueConsumer {
    pub async fn new() -> Result<Self> {
        let db = get_database().await?;
        let s3 = Arc::new(S3Service::new().await?);

        Ok(Self {
            files: FileService::new(&db, &s3)?,
            queue: QueueService::new(&db)?,
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
                self.files.resize_images(message.id).await?;
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
