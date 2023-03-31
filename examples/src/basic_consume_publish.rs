use tokio;
use tokio::sync::Notify;
use tokio::time;
use alicemq::consumer::{Consumer};
use alicemq::callback::{BaseCallbackConsumer};
use alicemq::publisher::Publisher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: Add tracing for publishing messages instead of prints.
    let new_event = "test_event".to_string();
    let new_callback = BaseCallbackConsumer {no_ack: false};
    let _ = Consumer::new()
        .connect()
        .await?
        .set_queue_manager()
        .build()
        .unwrap()
        .set_event_callback(new_event, new_callback)
        .start_consumer()
        .await?;

    let data = "{data: value}";

    //Giving time to bind queues to the consumer.
    time::sleep(time::Duration::from_secs(5)).await;

    let mut publisher = Publisher::new()
        .connect()
        .await.unwrap()
        .build()
        .unwrap();

    for _ in 1 .. 10 {
        publisher.clone().send_message("test_event".to_string(), data.to_string()).await;
    }
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
