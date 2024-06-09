use tracing::{debug, Level};
use alicemq::clients::consumer_client::ConsumerManager;
use alicemq::consumers::base_consumer::BaseConsumer;
use tracing_subscriber::FmtSubscriber;
use serde_json::{Value};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncWriteExt, BufWriter};

async fn my_callback(data: Value) {
    debug!("Receiving messages {:?}", &data);
    let filename = "output.json";
    let json_data = serde_json::to_vec(&data).expect("Failed to serialize data to JSON");
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)
        .await
        .expect("Failed to open file");

    let mut buf_writer = BufWriter::new(file);

    buf_writer.write_all(&json_data).await.expect("Failed to write data to file");
    buf_writer.flush().await.expect("Failed to flush buffer");
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