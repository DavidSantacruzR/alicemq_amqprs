use alicemq::clients::consumer_client::ConsumerManager;

#[tokio::main]
async fn main() {
    let mut _manager = ConsumerManager::new_instance();
    _manager.connect().await;

    // _manager.run().await;
}