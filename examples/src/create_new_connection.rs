use alicemq::consumer::{Consumer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let new_consumer = Consumer::new()
        .set_connection_arguments()?
        .set_event_queue("test".to_string());
    Ok(println!("{:?}", new_consumer))
}