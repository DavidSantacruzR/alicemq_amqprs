use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use tokio;
use alicemq::consumer::ConsumerManager;

struct ConsumerCallback;

impl AsyncConsumer for ConsumerCallback {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        _content: Vec<u8>,
    ) {
        #[cfg(feature = "tracing")]
        info!("consume delivery {} on channel {}", deliver, channel);

        // ack explicitly if manual ack
        if !self.no_ack {
            #[cfg(feature = "tracing")]
            info!("ack to delivery {} on channel {}", deliver, channel);
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    let mut test_consumer = ConsumerManager::new()
        .connect()
        .await
        .add_channel()
        .await
        .build();

    test_consumer.set_event_queue(
        "test_event".to_string(),
        ConsumerCallback
    );
    test_consumer.run(true).await;
}