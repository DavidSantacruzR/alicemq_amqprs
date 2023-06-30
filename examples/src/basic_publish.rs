use alicemq::{publisher::Publisher};
use tokio;

#[tokio::main]
async fn main() {

    let publisher = Publisher {};
    let message = String::from("data: {field_1: some data}");
    publisher.send_message(message, "test_event".to_string()).await;
}