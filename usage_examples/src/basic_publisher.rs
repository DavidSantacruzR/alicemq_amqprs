use tracing::{debug, Level};
use tracing_subscriber::FmtSubscriber;
use alicemq::clients::publisher_client::Publisher;
use serde_json::{json};

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
    for i in 1..2 {
        let data_obj = json!({
            "customer_id": i
        });
        debug!("This message contains the id {:?}", &data_obj);
        let _ = Publisher::send_message(data_obj, String::from("test_queue")).await;
    }
}