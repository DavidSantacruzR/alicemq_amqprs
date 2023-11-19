use async_trait::async_trait;
use tokio;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use alicemq::base::BaseCallback;
use alicemq::consumer::ConsumerManager;
use alicemq::enums::Runtime;

#[async_trait]
pub trait Runner {
    async fn run(&self, _message: String) {}
}


#[async_trait]
impl Runner for BaseCallback {
    async fn run(&self, _message: String) {
        println!("handling message");
        println!("{}", _message);
    }
}


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    let queue: String = "test_event".to_string();
    let callback = BaseCallback { runtime: Runtime::ASYNCHRONOUS};
    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .build();

    test_consumer
        .set_event_queue(
            queue,
            callback
        ).await
        .run(true).await;

}