use amqprs::{connection::Connection, channel::Channel, BasicProperties};
use amqprs::channel::{BasicPublishArguments, QueueBindArguments};
use amqprs::connection::OpenConnectionArguments;
use crate::callbacks::{CustomChannelCallback, CustomConnectionCallback};
use crate::settings::base::{Config};
use tokio::time;
use tracing::info;

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
    pub async fn send_message(mut self, data: String, queue: String) {
        let new_channel = self.connection.as_ref().expect("Unable to get connection")
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();

        let delivered_content = data.clone().into_bytes();
        let publishing_args = BasicPublishArguments::new("amq.topic", "amqprs.example");
        let channel = self.connection.open_channel(None).await.unwrap();
        new_channel
            .queue_bind(QueueBindArguments::new(
                &queue,
                "amp.topic",
                "amqprs.example"
            ))
            .await
            .unwrap();
        channel
            .register_callback(CustomConnectionCallback)
            .await
            .unwrap();
        channel
            .basic_publish(BasicProperties::default(), delivered_content, publishing_args)
            .await
            .unwrap();
        time::sleep(time::Duration::from_millis(100)).await;
        info!("message sent with data: {:?}", data);
    }
}
