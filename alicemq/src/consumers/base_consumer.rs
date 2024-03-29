use std::future::Future;
use std::marker::PhantomData;
use amqprs::channel::{Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;

pub struct BaseConsumer<F, Fut>
    where
        F: Fn(Vec<u8>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
{
    callback: F,
    _phantom: PhantomData<(Vec<u8>, Fut)>
}

impl<F, Fut> BaseConsumer<F, Fut>
    where
        F: Fn(Vec<u8>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
{
    pub fn new(callback: F) -> Self {
        BaseConsumer {
            callback,
            _phantom: PhantomData
        }
    }
}

#[async_trait]
impl<F, Fut> AsyncConsumer for BaseConsumer<F, Fut>
    where
        F: Fn(Vec<u8>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static
{
    async fn consume(
        &mut self,
        _channel: &Channel,
        _deliver: Deliver,
        _basic_properties: BasicProperties,
        content: Vec<u8>,
    ) {
        let callback = &self.callback;
        tokio::spawn(callback(content));
    }
}
