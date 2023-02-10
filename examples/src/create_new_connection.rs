use tokio;
use alicemq::consumer::{Consumer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_consumer = Consumer::new()
        .set_connection_arguments()?
        .set_event_queue("test".to_string()).expect("damn")
        .connect()
        .await?;
    Ok(())
}