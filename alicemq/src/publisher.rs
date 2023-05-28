use amqprs::{connection::Connection, BasicProperties};
use amqprs::channel::{BasicPublishArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::OpenConnectionArguments;
use crate::callbacks::{CustomChannelCallback, CustomConnectionCallback};
use crate::settings::base::{Config};
use tracing::info;

#[derive(Clone)]
pub struct Publisher {
    connection: Connection
}

impl Publisher {
    pub async fn connect() -> Publisher {
        let configuration = Config::new();
        let connection = Connection::open(&OpenConnectionArguments::new(
            &configuration.host,
            configuration.port,
            &configuration.username,
            &configuration.password
        ))
            .await
            .unwrap();
        connection
            .register_callback(CustomConnectionCallback)
            .await
            .unwrap();
        Publisher {
            connection,
        }
    }
    pub async fn send_message(self, data: String, queue: String) {
        let delivered_content = data.clone().into_bytes();
        let publishing_args = BasicPublishArguments::new("amq.topic", "amqprs.example");
        let new_channel = self.connection
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .queue_bind(QueueBindArguments::new(
                &queue,
                "amp.topic",
                "amqprs.example"
            ))
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        new_channel
            .basic_publish(BasicProperties::default(), delivered_content, publishing_args)
            .await
            .unwrap();
        info!("message sent with data: {:?}", data);
    }
}
