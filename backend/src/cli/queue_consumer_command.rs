//! This is the CLI entry point for the queue consumer.
//! The real work is done in the QueueConsumer class.

use crate::services::*;

pub async fn queue_consumer_command() {
    let locator = Locator::new();

    let consumer = locator
        .get::<QueueConsumer>()
        .expect("Error creating queue consumer.");

    consumer.run().await;
}
