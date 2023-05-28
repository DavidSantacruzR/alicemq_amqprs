use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use tokio;
use tracing::info;
use alicemq::{consumer::ConsumerManager, publisher::Publisher};
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
    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .add_channel()
        .await
        .build();

    let data_to_send: String = "data: {'field': 'name'}".to_string();
    let queue: String = "test_event".to_string();

    test_consumer
        .set_event_queue(
            "test_event".to_string(),
            ConsumerCallback {no_ack: false}
        ).await
        .run(true).await;

    let _ = Publisher::connect()
        .await
        .send_message(data_to_send.clone(), queue.clone())
        .await;
}