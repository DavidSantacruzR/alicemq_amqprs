use amqprs::channel::Channel;
use tokio::sync::Notify;
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::callbacks::{CustomConnectionCallback, CustomChannelCallback, CallbackRunner};
use crate::settings::base::{Config};

pub struct ConsumerManager {
    connection: Connection,
    channel: Channel,
    callback_runner: CallbackRunner,
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
            callback_runner: CallbackRunner{}
        }
    }
}

impl ConsumerManager {
    pub fn set_event_queue(&mut self) {}
    pub async fn run(&mut self) {
        let guard = Notify::new();
        guard.notified().await;
    }
}