use amqprs::{channel::Channel};
use amqprs::channel::{BasicAckArguments, BasicConsumeArguments, BasicQosArguments, ConsumerMessage, QueueBindArguments, QueueDeclareArguments};
use tokio::sync::{mpsc, Notify};
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::callbacks::{CustomConnectionCallback, CustomChannelCallback};
use crate::settings::base::{Config};
use amqprs::consumer::{AsyncConsumer};
use tracing::{info};
use crate::base::BaseCallback;
use crate::traits::Runner;

pub struct ConsumerManager {
    connection: Connection,
    channels: Vec<Channel>
}

pub struct ConsumerBuilder {
    config: Config,
    connection: Option<Connection>
}

impl ConsumerManager {
    pub fn new() -> ConsumerBuilder {
        ConsumerBuilder {
            config: Config::new(),
            connection: None
        }
    }

    pub async fn set_event_queue(mut self, event_name: String, callback: BaseCallback) -> Self {
        let new_channel = self.connection
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        let (queue_name, _, _) = new_channel
            .queue_declare(QueueDeclareArguments::new(&event_name))
            .await
            .unwrap()
            .unwrap();
        new_channel.queue_bind(QueueBindArguments::new(
            &queue_name,
            "amq.topic",
            "amqprs.example"
        )).await.unwrap();


        let another_channel = self.connection
            .open_channel(None)
            .await
            .unwrap();
        new_channel
            .register_callback(CustomChannelCallback)
            .await
            .unwrap();
        let (queue_name, _, _) = new_channel
            .queue_declare(QueueDeclareArguments::new(&event_name))
            .await
            .unwrap()
            .unwrap();
        new_channel.queue_bind(QueueBindArguments::new(
            &queue_name,
            "amq.topic",
            "amqprs.example"
        )).await.unwrap();


        let args = BasicConsumeArguments::new(
            &queue_name,
            &event_name
        )
            .no_ack(false)
            .finish();

        let (_ctag, mut messages_rx) =
            new_channel.basic_consume_rx(args).await.unwrap();

        tokio::spawn(async move {
            while let Some(message) = messages_rx.recv().await {
                let data = message.content.unwrap();
                info!("received message: {:?}", String::from_utf8(data.clone()));
                let _ = callback.run(String::from_utf8(data).unwrap());
                let ack_args = BasicAckArguments::new(
                    message.deliver.unwrap().delivery_tag(), false);
                another_channel.basic_ack(ack_args).await.unwrap();
            }
            return 0;
        }).await.unwrap();
        self.channels.push(new_channel);
        self
    }

    pub async fn run(self, long_lived: bool) {
        if long_lived {
            info!("started long lived consumer");
            let guard = Notify::new();
            guard.notified().await;
        } else {
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

    pub fn build(self) -> ConsumerManager {
        ConsumerManager {
            connection: self.connection.unwrap(),
            channels: vec!()
        }
    }
}
