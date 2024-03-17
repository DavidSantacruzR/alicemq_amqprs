use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use alicemq::clients::publisher_client::Publisher;

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
    for i in 1..10 {
        let _ = Publisher::send_message(
            format!("This message containse the id {}", i), String::from("test_queue")
        ).await;
    }
}