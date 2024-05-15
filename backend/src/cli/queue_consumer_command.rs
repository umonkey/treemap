use crate::services::QueueConsumer;

pub async fn queue_consumer_command() {
    let consumer = QueueConsumer::new()
        .await
        .expect("Error creating queue consumer.");

    consumer.run().await;
}
