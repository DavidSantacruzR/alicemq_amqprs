use tokio;
use tokio::sync::Notify;
use tokio::time;
use tracing::info;
use alicemq::consumer::{Consumer};
use alicemq::callback::{BaseCallbackConsumer};
use alicemq::publisher::Publisher;

async fn simple_callback(message: String) {
    info!("received a message {:?}", message)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_event = "test_event".to_string();
    let mut new_callback = BaseCallbackConsumer {no_ack: false, handler: Box::new((simple_callback))};
    let mut consumer = Consumer::new()
        .connect()
        .await?
        .set_queue_manager()
        .build()
        .unwrap()
        .set_event_callback(new_event, new_callback);

    consumer
        .start_consumer()
        .await?;

    let data = "{data: value}";

    time::sleep(time::Duration::from_secs(5)).await;

    let publisher = Publisher::new()
        .connect()
        .await.unwrap()
        .build()
        .unwrap();

    for _ in 1 .. 10 {
        publisher.clone().send_message("test_event".to_string(), data.to_string()).await;
    }
    publisher.close().await;
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
