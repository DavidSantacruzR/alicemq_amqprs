use amqprs::{connection::Connection, BasicProperties};
use amqprs::channel::{BasicPublishArguments, QueueBindArguments};
use amqprs::connection::OpenConnectionArguments;
use tokio::time;
use crate::callbacks::{CustomChannelCallback, CustomConnectionCallback};
use crate::settings::base::{Config};
use tracing::{info};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

#[derive(Clone)]
pub struct Publisher;

impl Publisher {
    pub async fn send_message(self, data: String, queue: String) {
        tracing_subscriber::registry();

        let message = data.into_bytes();
        let routing_key = "amqprs.example";
        let exchange_name = "amq.topic";
        let connection_parameters = Config::new();
        let connection = Connection::open(&OpenConnectionArguments::new(
            &connection_parameters.host,
            connection_parameters.port,
            &connection_parameters.username,
            &connection_parameters.password,
        ))
            .await
            .unwrap();
        connection
            .register_callback(CustomConnectionCallback)
            .await
            .unwrap();

        let channel = connection.open_channel(None).await.unwrap();
        channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        channel
            .queue_bind(QueueBindArguments::new(
                &queue,
                exchange_name,
                routing_key,
            ))
            .await
            .unwrap();

        let args = BasicPublishArguments::new(exchange_name, routing_key);
        info!("Sending message with data {:?}", &message);
        channel
            .basic_publish(BasicProperties::default(), message, args)
            .await
            .unwrap();
        time::sleep(time::Duration::from_secs(1)).await;
        info!("message delivered successfully.");
        channel.close().await.unwrap();
        connection.close().await.unwrap();
    }
}
