use tokio;
use tokio::sync::Notify;
use tokio::time;
use alicemq::consumer::{Consumer};
use alicemq::callback::{BaseCallbackConsumer};
use alicemq::publisher::Publisher;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_event = "test_event_manual_ack".to_string();

    //In the callback set whether you want auto acknowledgement of messages.
    let new_callback = BaseCallbackConsumer::new(true);
    let mut consumer = Consumer::new()
        .connect()
        .await?
        .set_queue_manager()
        .build()
        .unwrap()
        .set_event_callback(new_event, new_callback);

    /*Starts the consumer, and keep it running, if out scope closes active connections.*/
    consumer
        .start_consumer()
        .await?;

    let data = "{data: {id: 1, value: None}";

    time::sleep(time::Duration::from_secs(5)).await;

    let publisher = Publisher::new()
        .connect()
        .await.unwrap()
        .build()
        .unwrap();

    for _ in 1 .. 10 {
        publisher.clone().send_message("test_event_manual_ack".to_string(), data.to_string()).await;
    }
    publisher.close().await;
    let guard = Notify::new();
    guard.notified().await;
    Ok(())
}
