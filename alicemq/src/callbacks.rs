
use amqprs::{Ack, BasicProperties, callbacks::{ConnectionCallback, ChannelCallback}, Cancel, Close, CloseChannel, connection::{Connection}, Nack, Return};
use amqprs::channel::Channel;
use async_trait::async_trait;
use tracing::{error, info, warn};

type Result<T> = std::result::Result<T, amqprs::error::Error>;

pub struct CustomConnectionCallback;

#[async_trait]
impl ConnectionCallback for CustomConnectionCallback {
    async fn close(&mut self, connection: &Connection, close: Close) -> Result<()> {
        error!("close request for connection: {}, reason: {}", connection, close);
        Ok(())
    }
    async fn blocked(&mut self, connection: &Connection, reason: String) {
        info!("blocked connection: {}, reason: {}", connection, reason)
    }
    async fn unblocked(&mut self, connection: &Connection) {
        info!("unblocked notifications for connection: {}", connection)
    }
}

pub struct CustomChannelCallback;

#[async_trait]
impl ChannelCallback for CustomChannelCallback {
    async fn close(&mut self, channel: &Channel, close: CloseChannel) -> Result<()> {
        error!("closing channel {}, reason: {}", channel, close);
        Ok(())
    }
    async fn cancel(&mut self, channel: &Channel, cancel: Cancel) -> Result<()> {
        warn!("Handling cancel request for consumer: {}, on channel {}",
            cancel.consumer_tag(),
            channel
        );
        Ok(())
    }
    async fn flow(&mut self, channel: &Channel, active:bool) -> Result<bool> {
        info!("Flow request for channel {}: {}", channel, active);
        Ok(true)
    }
    async fn publish_ack(&mut self, channel: &Channel, ack: Ack) {
        info!("Publish ack for channel {}: {}", channel, ack.delivery_tag());
    }
    async fn publish_nack(&mut self, channel: &Channel, nack: Nack) {
        warn!("handle publish nack delivery_tag: {} on channel {}", nack.delivery_tag(), channel);
    }
    async fn publish_return(
        &mut self,
        channel: &Channel,
        ret: Return,
        _basic_properties: BasicProperties,
        content: Vec<u8>
    ) {
        warn!("handle publish return {} on channel {}, with message size {}",
            ret,
            channel,
            content.len()
        );
    }
}
