use amqprs::channel::Channel;
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;

pub struct BaseCallbackConsumer {
    pub no_ack: bool
}

impl BaseCallbackConsumer {
    pub fn new(no_ack: bool) -> Self {
        Self { no_ack }
    }
}

#[async_trait]
impl AsyncConsumer for BaseCallbackConsumer {
    async fn consume(&mut self,
                     channel: &Channel,
                     deliver: Deliver,
                     _basic_properties: BasicProperties,
                     _content: Vec<u8>) {
        println!("the content of the message is {:?}", _content)
    }
}
