use crate::services::*;

pub async fn queue_consumer_command() {
    let locator = Locator::new();

    let consumer = locator
        .get::<QueueConsumer>()
        .expect("Error creating queue consumer.");

    consumer.run().await;
}
