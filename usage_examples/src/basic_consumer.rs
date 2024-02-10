use std::future::Future;
use alicemq::clients::consumer_client::ConsumerManager;
use alicemq::consumers::base_consumer::BaseConsumer;

async fn my_callback(data: Vec<u8>) -> impl Future<Output = ()> {
    println!("Received data: {:?}", data);
    // Placeholder future
    async {}
}

#[tokio::main]
async fn main() {
    let mut _manager: ConsumerManager = ConsumerManager::new_instance()
        .connect().await;
    _manager.set_queue("test_queue", BaseConsumer::new(
        true,
        |data| {
            async move {
                my_callback(data).await.await;
            }
        },
    )).await;
    _manager.set_queue("another_test_queue", BaseConsumer::new(
        true,
        |data| {
            async move {
                my_callback(data).await.await;
            }
        },
    )).await;
    _manager.run(true).await;
}