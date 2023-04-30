use tokio;
use alicemq::consumer::ConsumerManager;


#[tokio::main]
async fn main() {
    let mut test_consumer = ConsumerManager::new()
        .connect()
        .await
        .add_channel()
        .await
        .build();

    test_consumer.set_event_queue();
    test_consumer.run().await;
}