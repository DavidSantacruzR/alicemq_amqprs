use alicemq::clients::consumer_client::ConsumerManager;

#[tokio::main]
async fn main() {
    let mut _manager: ConsumerManager = ConsumerManager::new_instance()
        .connect().await;
    _manager.run(true).await;
}