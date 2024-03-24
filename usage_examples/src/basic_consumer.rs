use tracing::{debug, Level};
use alicemq::clients::consumer_client::ConsumerManager;
use alicemq::consumers::base_consumer::BaseConsumer;
use tracing_subscriber::FmtSubscriber;

async fn my_callback(data: Vec<u8>) {
    debug!("Received data: {:?}", String::from_utf8(data));
}

fn set_tracing_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    let _ = set_tracing_subscriber();
    let mut _manager: ConsumerManager = ConsumerManager::new_instance()
        .connect().await;
    _manager.set_queue("test_queue", BaseConsumer::new(my_callback), None).await;
    _manager.set_queue("another_test_queue", BaseConsumer::new(my_callback), None).await;
    _manager.run(true).await;
}