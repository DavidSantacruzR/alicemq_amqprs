use std::collections::HashMap;
use amqprs::callbacks::ConnectionCallback;
use amqprs::channel::Channel;
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::callbacks::{CustomConnectionCallback, CustomChannelCallback, CallbackRunner};
use crate::settings::base::{Config};

struct ConsumerManager {
    connection: Connection,
    channel: Channel,
    callback_runner: CallbackRunner,
}

struct ConsumerBuilder {
    config: Config,
    connection: Option<Connection>
}

impl ConsumerManager {
    fn new() -> ConsumerBuilder {
        ConsumerBuilder {
            config: Config::new(),
            connection: None
        }
    }
}

impl ConsumerBuilder {

    async fn connect<F>(&mut self, callback: F) where F: ConnectionCallback + Send + 'static {
        let connection = Connection::open(&OpenConnectionArguments::new(
            &self.config.host,
            self.config.port,
            &self.config.username,
            &self.config.password
        ))
            .await
            .unwrap();
        connection
            .register_callback(callback)
            .await
            .unwrap();
        self.connection = Some(connection);
    }
}

impl ConsumerManager {
    async fn add_channel(&mut self) {
        let mut new_channel = self.connection
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        self.channel = new_channel;
    }
}