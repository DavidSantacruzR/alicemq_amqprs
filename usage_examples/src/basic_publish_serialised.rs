use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use alicemq::clients::publisher_client::Publisher;
use serde_json::json;

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
    let data = json!({
        "name": "test",
        "value": 100
    });
    for _ in 1..5 {
        let _ = Publisher::send_message(
            data.to_string().clone(), String::from("test_queue")
        ).await;
    }
}