use std::collections::HashMap;
use amqprs::connection::{Connection, OpenConnectionArguments};
use std::default::Default;
use std::error::Error;
use callback::BaseCallback;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, QueueBindArguments, QueueDeclareArguments};
use amqprs::consumer::DefaultConsumer;
use dotenv::dotenv;
use crate::callback;
use tokio::sync::Notify;
/*
Creating a connection to a rabbitMQ server.
*/
#[derive(Debug)]
pub struct ConnectionArguments {
    host: String,
    port: u16,
    username: String,
    password: String
}

impl Default for ConnectionArguments {
    fn default() -> Self {
        ConnectionArguments {
            host: "localhost".to_string(),
            port: 5672,
            username: "guest".to_string(),
            password: "guest".to_string(),
        }

    }
}

/*
Default implementation of the consumer, queue manager and its respective builder.
*/

pub struct Consumer {
    connection: Connection,
    pub queue_manager: HashMap<String, BaseCallback>
}

impl Consumer {
    pub fn new() -> ConsumerBuilder {
        ConsumerBuilder::default()
    }
}

#[derive(Default)]
pub struct ConsumerBuilder {
    connection: Option<Connection>,
    connection_arguments: Option<ConnectionArguments>,
    queue_manager: Option<HashMap<String, BaseCallback>>,
}

impl ConsumerBuilder {
    pub fn set_connection_arguments(mut self) -> Result<Self, Box<dyn Error>> {
        dotenv().ok();
        let host: String = std::env::var("host").expect("No host set.");
        let port: u16 = std::env::var("port").expect("No port set.").parse::<u16>().unwrap();
        let username: String = std::env::var("username").expect("No user set.");
        let password: String = std::env::var("password").expect("No pwd set.");
        self.connection_arguments.get_or_insert(ConnectionArguments {
            host,
            port,
            username,
            password
        });
        Ok(self)
    }

    pub async fn connect(mut self) -> Result<Self, Box<dyn Error>> {
        self.connection.get_or_insert(Connection::open( &OpenConnectionArguments::new(
            &self.connection_arguments.as_ref().unwrap().host,
            self.connection_arguments.as_ref().unwrap().port,
            &self.connection_arguments.as_ref().unwrap().username,
            &self.connection_arguments.as_ref().unwrap().password
        )).await.unwrap());
        self.connection.as_ref().ok_or("Unable to open a connection to RabbitMQ cluster.")?
            .register_callback(DefaultConnectionCallback)
            .await
            .unwrap();
        Ok(self)
    }
    pub fn set_queue_manager(mut self) -> Self {
        self.queue_manager.get_or_insert(HashMap::new());
        self
    }
    pub fn set_event_callback(mut self, event_queue: String, callback: BaseCallback) -> Self {
        self.queue_manager.as_mut().unwrap().insert(
            event_queue,
            callback
        );
        self
    }
    pub async fn start_consumer(self) -> Result<Consumer, Box<dyn Error>> {
        //Traverses the hashmap, and creates a respective, queue and channel for each event.
        for (event, callback) in self.queue_manager.as_ref()
            .ok_or("Error binding event/callback to manager.")? {
            let channel = self.connection.as_ref().
                expect("Unable to set channel").
                open_channel(None).
                await.
                unwrap();
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
        }
        println!("consuming forever..., ctrl+c to exit");
        let guard = Notify::new();
        guard.notified().await;
        Ok(Consumer {
            connection: self.connection.ok_or("Unable to set a connection to rabbitMQ.")?,
            queue_manager: self.queue_manager.ok_or("Queue manager not currently active.")?
        })
    }
    //TODO: Add custom error handling on consumer start.
}
