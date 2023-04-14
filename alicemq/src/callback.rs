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
    pub no_ack: bool,
    pub handler: Box<dyn FnMut()>
}

impl BaseCallbackConsumer {
    pub fn set_callback(&mut self, callback_handler:impl FnMut() + 'static) {
        self.handler = callback_handler;
    }
    async fn handle_message(&mut self, message: String) {
        (self.handler)(message).await;
    }
}

#[async_trait]
impl AsyncConsumer for BaseCallbackConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let message = std::str::from_utf8(&content).unwrap();
        self.handle_message(message.to_string());
        if !self.no_ack {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}
