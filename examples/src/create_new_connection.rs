use tokio;
use alicemq::consumer::{Consumer};
use alicemq::callback::{BaseCallback};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let new_event = "test_event".to_string();
    let new_callback = BaseCallback;
    let another_event = "another_event".to_string();
    let another_callback = BaseCallback;
    let _ = Consumer::new()
        .set_connection_arguments()?
        .connect()
        .await?
        .set_queue_manager()
        .set_event_callback(new_event, new_callback)
        .set_event_callback(another_event, another_callback)
        .start_consumer()
        .await?;
    Ok(())
}
