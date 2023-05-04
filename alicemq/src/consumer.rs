use std::collections::HashMap;
use amqprs::{channel::Channel};
use tokio::sync::Notify;
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::callbacks::{CustomConnectionCallback, CustomChannelCallback};
use crate::settings::base::{Config};
use amqprs::consumer::AsyncConsumer;

type BaseCallback = Box< dyn AsyncConsumer + Send + 'static>;

pub struct ConsumerManager {
    connection: Connection,
    channel: Channel,
    callback_runner: HashMap<String, BaseCallback>
}

pub struct ConsumerBuilder {
    config: Config,
    channel: Option<Channel>,
    connection: Option<Connection>
}

impl ConsumerManager {
    pub fn new() -> ConsumerBuilder {
        ConsumerBuilder {
            config: Config::new(),
            channel: None,
            connection: None
        }
    }
    pub fn set_event_queue(&mut self, event_name: String, callback: BaseCallback) {
        self.callback_runner.insert(
            event_name,
            callback
        );
    }
    pub async fn run(&mut self, long_lived: bool) {
        //Runs all task at hand calling their custom callback consumers.
        if long_lived {
            let guard = Notify::new();
            guard.notified().await;
        }
    }
}

impl ConsumerBuilder {

    pub async fn connect(mut self) -> Self {
        let connection = Connection::open(&OpenConnectionArguments::new(
            &self.config.host,
            self.config.port,
            &self.config.username,
            &self.config.password
        ))
            .await
            .unwrap();
        connection
            .register_callback(CustomConnectionCallback)
            .await
            .unwrap();
        self.connection = Some(connection);
        self
    }

    pub async fn add_channel(mut self) -> Self {
        let new_channel = self.connection.as_ref().expect("No connection set.")
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        self.channel = Some(new_channel);
        self
    }

    pub fn build(self) -> ConsumerManager {
        ConsumerManager {
            connection: self.connection.unwrap(),
            channel: self.channel.unwrap(),
            callback_runner: HashMap::new()
        }
    }
}
