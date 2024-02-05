use amqprs::channel::Channel;
use amqprs::connection::{Connection, OpenConnectionArguments};
use tokio::sync::Notify;
use tracing::{debug};
use crate::settings::configuration::ConnectionSettings;

#[allow(dead_code)]
pub struct ConsumerManager {
    connection: Connection,
    channels : Vec<Channel>
}

impl ConsumerManager {

    pub fn new_instance() -> ConsumerBuilder {
        ConsumerBuilder {}
    }

    pub async fn run(self, long_lived: bool) {
        if long_lived {
            debug!("Consuming messages from queue.");
            let _guard = Notify::new();
            _guard.notified().await;
        } else {
            for channel in self.channels {
                channel.close().await.unwrap();
            }
            self.connection.close().await.unwrap();
        }
    }
}

pub struct ConsumerBuilder;

impl ConsumerBuilder {

    pub async fn connect(self) -> ConsumerManager {
        let _settings = ConnectionSettings::new();
        let _connection = Connection::open(&OpenConnectionArguments::new(
            &_settings.host,
            _settings.port,
            &_settings.username,
            &_settings.password
        )).await.unwrap();
        ConsumerManager { connection: _connection, channels: vec!() }
    }
}


