use tokio;
use alicemq::consumer::{Consumer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_consumer = Consumer::new()
        .set_connection_arguments()?
        .connect()
        .await?
        .set_queue_manager()
        .start_consumer()
        .await?;
    Ok(println!("event manager {:?}", new_consumer.queue_manager))
}