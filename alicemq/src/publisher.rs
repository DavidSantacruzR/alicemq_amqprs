use amqprs::{connection::Connection, BasicProperties};
use amqprs::channel::{BasicPublishArguments, QueueBindArguments};
use amqprs::connection::OpenConnectionArguments;
use crate::callbacks::{CustomChannelCallback, CustomConnectionCallback};
use crate::settings::base::{Config};
use tracing::info;

#[derive(Clone, Default)]
pub struct Publisher {
    connection: Option<Connection>
}

impl Publisher {
    pub async fn connect(mut self) -> Self {
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
        self.connection = Some(connection);
        self
    }
    pub async fn send_message(self, data: String, queue: String) {
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
        let channel = self.connection.expect("No connection set to open.").open_channel(None).await.unwrap();
        new_channel
            .queue_bind(QueueBindArguments::new(
                &queue,
                "amp.topic",
                "amqprs.example"
            ))
            .await
            .unwrap();
        channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        channel
            .basic_publish(BasicProperties::default(), delivered_content, publishing_args)
            .await
            .unwrap();
        info!("message sent with data: {:?}", data);
    }
}
