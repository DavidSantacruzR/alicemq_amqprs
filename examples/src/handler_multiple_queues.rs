use tokio;
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;
use alicemq::consumer::ConsumerManager;

fn handler_1(message: String) {
    println!("handler_1: {}", message);
}

fn handler_2(message: String) {
    println!("handler_2: {}", message);
}

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .build();

    test_consumer
        .set_event_queue(
            String::from("queue_1"),
            handler_1
        ).await
        .set_event_queue(
            String::from("queue_2"),
            handler_2
        ).await
        .run(true).await;
}