use super::schemas::*;
use crate::domain::file::FileService;
use crate::domain::panorama::{PanoramaLogs, PanoramaService};
use crate::domain::photo::PhotoService;
use crate::domain::tree::TreeService;
use crate::domain::user::*;
use crate::infra::queue::{Queue, QueueMessage};
use crate::services::{Context, Injectable};
use crate::types::*;
use crate::utils::get_timestamp;
use crate::utils::metrics::{record_queue_message, record_queue_retry};
use log::{debug, error, info};
use std::sync::Arc;
use std::time::{Duration, Instant};

// Seconds to wait for a new message.
const DELAY: u64 = 1;

pub struct QueueConsumer {
    queue: Arc<Queue>,
    trees: Arc<TreeService>,
    photos: Arc<PhotoService>,
    users: Arc<UserService>,
    panoramas: Arc<PanoramaService>,
    panorama_logs: Arc<PanoramaLogs>,
    files: Arc<FileService>,
}

impl QueueConsumer {
    pub async fn run(&self) {
        info!("Running queue consumer.");

        loop {
            let msg = self.queue.pop().await;

            match msg {
                Ok(Some(msg)) => {
                    let waited = Duration::from_secs(get_timestamp().saturating_sub(msg.added_at));

                    match QueueCommand::decode(&msg.payload) {
                        Ok(Some(command)) => {
                            let name = command.name();

                            if msg.attempts > 0 {
                                record_queue_retry(name);
                            }

                            let started = Instant::now();
                            let result = self.process_message(command).await;
                            let elapsed = started.elapsed();
                            let outcome = if result.is_ok() { "ok" } else { "error" };
                            let error = result.as_ref().err().map(error_kind);

                            record_queue_message(name, outcome, error, waited, elapsed);

                            match result {
                                Ok(_) => self.delete_message(&msg).await,
                                Err(e) => self.delay_message(&e, &msg).await,
                            };
                        }

                        Ok(None) => {
                            debug!("Unknown message: {}", msg.payload);
                            self.delete_message(&msg).await;
                        }

                        Err(e) => {
                            error!("Error while decoding message: {e:?}");
                            self.delete_message(&msg).await;
                        }
                    }
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
     * Process a decoded queue command.
     */
    async fn process_message(&self, command: QueueCommand) -> Result<()> {
        match command {
            QueueCommand::ProcessFile(message) => {
                self.files.process(message.id).await?;
            }

            QueueCommand::UpdateTreeAddress(message) => {
                self.trees.update_tree_address(message.id).await?;
            }

            QueueCommand::AddPhoto(message) => {
                self.photos
                    .add_tree_photo(message.tree_id, message.file_id)
                    .await?;
            }

            QueueCommand::UpdateUserpic(message) => {
                self.users
                    .update_userpic(message.user_id, message.file_id)
                    .await?;
            }

            QueueCommand::UpdatePanoramaStats(message) => {
                info!(
                    "Processing UpdatePanoramaStats for panorama {}.",
                    message.id
                );
                self.panoramas.refresh_panorama_stats(message.id).await?;
            }

            QueueCommand::DownloadPanoramaLogs(message) => {
                info!(
                    "Processing DownloadPanoramaLogs for panorama {}.",
                    message.id
                );
                self.panorama_logs
                    .download(message.id, &message.log_stream)
                    .await?;
            }
        }

        Ok(())
    }
}

/// Bounded failure kind for the `queue.error` metric attribute.
fn error_kind(error: &Error) -> &'static str {
    if error.is_transient_lock() {
        "locked"
    } else if matches!(error, Error::DatabaseQuery(_)) {
        "database"
    } else {
        "other"
    }
}

impl Injectable for QueueConsumer {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            queue: ctx.queue(),
            trees: Arc::new(ctx.build::<TreeService>()?),
            photos: Arc::new(ctx.build::<PhotoService>()?),
            users: Arc::new(ctx.build::<UserService>()?),
            panoramas: Arc::new(ctx.build::<PanoramaService>()?),
            panorama_logs: Arc::new(ctx.build::<PanoramaLogs>()?),
            files: Arc::new(ctx.build::<FileService>()?),
        })
    }
}
