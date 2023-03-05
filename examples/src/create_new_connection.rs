use tokio;
use alicemq::consumer::{Consumer};
use alicemq::callback::{BaseCallbackConsumer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let new_event = "test_event".to_string();
    let new_callback = BaseCallbackConsumer {no_ack: false};
    let another_event = "another_event".to_string();
    let another_callback = BaseCallbackConsumer {no_ack: false};
    let _ = Consumer::new()
        .connect()
        .await?
        .set_queue_manager()
        .build()
        .unwrap()
        .set_event_callback(new_event, new_callback)
        .set_event_callback(another_event, another_callback)
        .start_consumer()
        .await?;
    Ok(())
}
