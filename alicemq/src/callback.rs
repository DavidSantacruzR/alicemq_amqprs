use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use tracing::info;

/// The `BaseCallbackConsumer` is the main type expected by the consumer,
/// and it's where a custom message handling must be implemented.
/// AsyncConsumer as well as BlockingConsumer traits from the amqprs library are supported.

#[derive(Debug, Clone, Copy)]
pub struct BaseCallbackConsumer {
    /// A `True` value indicates that all message consumer acknowledgements must be implemented.
    pub no_ack: bool
}

impl BaseCallbackConsumer {
    pub fn new(no_ack: bool) -> Self {
        Self { no_ack }
    }
}

#[async_trait]
impl AsyncConsumer for BaseCallbackConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        _content: Vec<u8>,
    ) {
        info!("got message {:?}", std::str::from_utf8(&_content));
        if !self.no_ack {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}
