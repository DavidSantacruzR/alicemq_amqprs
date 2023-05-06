use amqprs::{channel::Channel};
use amqprs::channel::{BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments};
use tokio::sync::Notify;
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::callbacks::{CustomConnectionCallback, CustomChannelCallback};
use crate::settings::base::{Config};
use amqprs::consumer::AsyncConsumer;
use tracing::info;

pub struct ConsumerManager {
    connection: Connection,
    channel: Channel
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

    pub async fn set_event_queue<F>(self, event_name: String, callback: F) -> Self
        where F: AsyncConsumer + Send + 'static {
        let (queue_name, _, _) = self.channel
            .queue_declare(QueueDeclareArguments::new(&event_name))
            .await
            .unwrap()
            .unwrap();
        self.channel.queue_bind(QueueBindArguments::new(
            &queue_name,
            "amq.topic",
            "amqprs.example"
        )).await.unwrap();
        let args = BasicConsumeArguments::new(
            &queue_name,
            "basic_consumer"
        )
            .no_ack(false)
            .finish();
        self.channel
            .basic_consume(callback, args)
            .await
            .unwrap();
        self
    }

    pub async fn run(self, long_lived: bool) {
        if long_lived {
            info!("started long lived consumer");
            let guard = Notify::new();
            guard.notified().await;
        } else {
            self.channel.close().await.unwrap();
            self.connection.close().await.unwrap();
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
        let new_channel = self.connection.as_ref().expect("Unable to get connection")
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
            channel: self.channel.unwrap()
        }
    }
}
