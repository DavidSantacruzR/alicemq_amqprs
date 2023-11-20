use tokio;
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;
use alicemq::consumer::ConsumerManager;

fn print_some_stuff(message: String) {
    let id = Uuid::new_v4();
    println!("Got message: {}, stored with uuid: {}", message, id);
}

#[tokio::main]
async fn main() {

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let queue: String = "test_event".to_string();

    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .build();

    test_consumer
        .set_event_queue(
            queue,
            print_some_stuff
        ).await
        .run(true).await;
}