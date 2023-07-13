use amqprs::{channel::Channel};
use amqprs::channel::{
    BasicConsumeArguments, 
    BasicQosArguments, 
    QueueBindArguments, 
    QueueDeclareArguments
};
use tokio::sync::Notify;
use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::callbacks::{CustomConnectionCallback, CustomChannelCallback};
use crate::settings::base::{Config};
use amqprs::consumer::{AsyncConsumer};
use tracing::{info};

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

    pub async fn set_event_queue<F>(mut self, event_name: String, callback: F) -> Self
        where F: AsyncConsumer + Send + 'static {
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
        let args = BasicConsumeArguments::new(
            &queue_name,
            &event_name
        )
            .no_ack(false)
            .finish();
        new_channel.basic_qos(
            BasicQosArguments::new(
                0,0,false
            )
        )
            .await
            .unwrap();
        new_channel
            .basic_consume(callback, args)
            .await
            .unwrap();
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
