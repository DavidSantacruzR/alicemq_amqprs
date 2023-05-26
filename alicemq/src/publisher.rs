use amqprs::{connection::Connection, channel::Channel};
use amqprs::connection::OpenConnectionArguments;
use crate::callbacks::{CustomChannelCallback, CustomConnectionCallback};
use crate::settings::base::{Config};

#[derive(Clone)]
pub struct Publisher {
    connection: Connection,
    message_channel: Channel
}

impl Publisher {
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
        self.connection = connection;
        self
    }
    pub async fn send_message(mut self, data: String) {
        let new_channel = self.connection.as_ref().expect("Unable to get connection")
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();

        new_channel.send
    }
}



