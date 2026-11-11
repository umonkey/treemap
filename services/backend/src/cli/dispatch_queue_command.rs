use crate::services::queue_consumer::QueueConsumer;
use crate::services::{AppState, ContextExt};
use crate::types::Result;

pub async fn dispatch_queue_command() -> Result<()> {
    let state = AppState::new().await?;

    let consumer = state.build::<QueueConsumer>()?;

    consumer.run().await;

    Ok(())
}
