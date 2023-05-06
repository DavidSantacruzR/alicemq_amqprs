use std::collections::HashMap;
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
    channel: Channel,
    callback_runner: HashMap<String, Box<dyn AsyncConsumer + Send + 'static>>,
    queue_bindings: Vec<Channel>
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

    pub async fn set_event_queue<F>(&mut self, event_name: String, callback: F)
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
        let _ = self.callback_runner.insert(
            event_name,
            Box::new(callback)
        );
    }

    pub async fn run(&mut self, long_lived: bool) {
        //Runs all task at hand calling their custom callback consumers.
        //Creates channels for every event, callback pair.
        if long_lived {
            let guard = Notify::new();
            guard.notified().await;
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
            channel: self.channel.unwrap(),
            callback_runner: HashMap::new(),
            queue_bindings: vec![]
        }
    }
}
