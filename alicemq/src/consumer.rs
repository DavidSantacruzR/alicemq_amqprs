use std::collections::HashMap;
use amqprs::connection::{Connection};
use std::default::Default;
use std::error::Error;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments};
use amqprs::consumer::DefaultConsumer;
use tokio::sync::Notify;

use crate::{connection_arguments::*};
use crate::callback::BaseCallbackConsumer;

pub struct Consumer {
    connection: Connection,
    pub queue_manager: HashMap<String, BaseCallbackConsumer>,
    registered_channels: Vec<Channel>
}

impl Consumer {
    pub fn new() -> ConsumerBuilder {
        ConsumerBuilder::default()
    }
}

#[derive(Default)]
pub struct ConsumerBuilder {
    connection: Option<Connection>,
    queue_manager: Option<HashMap<String, BaseCallbackConsumer>>,
}

impl ConsumerBuilder {
    pub async fn connect(mut self) -> Result<Self, Box<dyn Error>> {
        self.connection.get_or_insert(Connection::open( &ConnectionArguments::load_from_env())
            .await
            .unwrap()
        );
        self.connection.as_ref().ok_or("Unable to open a connection to RabbitMQ cluster.".to_string())?
            .register_callback(DefaultConnectionCallback)
            .await
            .unwrap();
        Ok(self)
    }
    pub fn set_queue_manager(mut self) -> Self {
        self.queue_manager.get_or_insert(HashMap::new());
        self
    }
    pub fn build(self) -> Result<Consumer, Box<dyn Error>> {
        Ok(Consumer {
            connection: self.connection.ok_or("Unable to set a connection to rabbitMQ.")?,
            queue_manager: self.queue_manager.ok_or("Queue manager not currently active.")?,
            registered_channels: vec!()
        })
    }
    //TODO: Document the methods and their expected behaviour.
    //TODO: Add handlers to manage queues in case of panic.
    //TODO: Add custom error handling on consumer start, on callbacks.
}

impl Consumer {
    pub fn set_event_callback(mut self, event_queue: String, callback: BaseCallbackConsumer) -> Self {
        let _ = &self.queue_manager.insert(
            event_queue,
            callback
        );
        self
    }
    pub async fn start_consumer(mut self) -> Result<(), Box<dyn Error>> {
        for (event, callback) in &self.queue_manager {
            let channel = self.connection
                .open_channel(None)
                .await
                .unwrap();
            channel
                .register_callback(DefaultChannelCallback)
                .await
                .unwrap();
            let (queue_name, _, _) = channel
                .queue_declare(QueueDeclareArguments::new(event))
                .await
                .unwrap()
                .unwrap();
            let routing_key = "amqprs.example";
            let exchange_name = "amq.topic";
            channel
                .queue_bind(QueueBindArguments::new(
                    &queue_name,
                    exchange_name,
                    routing_key,
                ))
                .await
                .unwrap();
            let args = BasicConsumeArguments::new(&queue_name, "basic_consumer")
                .no_ack(true)
                .finish();
            channel
                .basic_consume(DefaultConsumer::new(false), args)
                .await
                .unwrap();
            &self.registered_channels.push(channel);
        }
        println!("consuming forever..., ctrl+c to exit");
        //TODO: add map event to specific data handler.
        let guard = Notify::new();
        guard.notified().await;
        Ok(())
    }
}
