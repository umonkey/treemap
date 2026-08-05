use crate::services::queue_consumer::QueueConsumer;
use crate::services::{AppState, ContextExt};

pub async fn dispatch_queue_command() {
    let state = AppState::new()
        .await
        .expect("Error initializing app state.");

    let consumer = state
        .build::<QueueConsumer>()
        .expect("Error creating queue consumer.");

    consumer.run().await;
}
