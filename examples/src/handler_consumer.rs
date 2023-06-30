use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use tokio;
use tracing::info;
use alicemq::{consumer::ConsumerManager};
use async_trait::async_trait;

struct ConsumerCallback {
    no_ack: bool
}

#[async_trait]
impl AsyncConsumer for ConsumerCallback {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        _content: Vec<u8>,
    ) {
        info!("got message with data {}", std::str::from_utf8(&_content).unwrap());
        if !self.no_ack {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {

    let queue: String = "test_event".to_string();

    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .add_channel()
        .await
        .build();

    test_consumer
        .set_event_queue(
            queue,
            ConsumerCallback {no_ack: false}
        ).await
        .run(true).await;

}