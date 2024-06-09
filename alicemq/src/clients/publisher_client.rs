
use amqprs::{connection::Connection, BasicProperties};
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicPublishArguments, QueueBindArguments};
use amqprs::connection::OpenConnectionArguments;
use tracing::{debug};
use crate::settings::configuration::ConnectionSettings;
use serde_json::{Value};
use serde_json::to_vec;

#[derive(Clone)]
pub struct Publisher;

impl Publisher {
    pub async fn send_message(data: Value, queue: String) {
        tracing_subscriber::registry();

        let message = to_vec(&data).unwrap();
        let routing_key = "amqprs.example";
        let exchange_name = "amq.topic";
        let _settings = ConnectionSettings::new();
        let _connection = Connection::open(&OpenConnectionArguments::new(
            &_settings.host,
            _settings.port,
            &_settings.username,
            &_settings.password
        )).await.unwrap();
        _connection
            .register_callback(DefaultConnectionCallback)
            .await
            .unwrap();

        let channel = _connection.open_channel(None).await.unwrap();
        channel
            .register_callback(DefaultChannelCallback)
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
        debug!("Sending message with data {:?}", String::from_utf8(message.clone()));
        channel
            .basic_publish(BasicProperties::default(), message, args)
            .await
            .unwrap();
        debug!("message delivered successfully.");
        channel.close().await.unwrap();
        _connection.close().await.unwrap();
    }
}