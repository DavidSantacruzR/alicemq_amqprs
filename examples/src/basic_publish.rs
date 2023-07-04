use alicemq::{publisher::Publisher};
use tokio;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let publisher = Publisher {};
    let message = String::from("data: {field_1: some data}");
    publisher.send_message(message, "test_event".to_string()).await;
}