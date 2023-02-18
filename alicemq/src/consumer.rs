use std::collections::HashMap;
use amqprs::connection::{Connection, OpenConnectionArguments};
use std::default::Default;
use std::error::Error;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{Channel, QueueBindArguments, QueueDeclareArguments};
use dotenv::dotenv;

#[derive(Debug)]
pub struct BaseCallback;

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
    pub async fn create_channel(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let channel = self.connection.as_ref().unwrap().open_channel(None).await.unwrap();
        channel
            .register_callback(DefaultChannelCallback)
            .await
            .unwrap();
        Ok(self)
    }
    pub fn set_queue_manager(mut self) -> Self {
        self.queue_manager.get_or_insert(HashMap::new());
        self
    }
    pub fn set_event_callback(mut self, event_queue: String, callback: BaseCallback) -> Self {
        //TODO: Create a call to queue create here.
        self.queue_manager.as_mut().unwrap().insert(
            event_queue,
            callback
        );
        self
    }
    pub async fn start_consumer(self) -> Result<Consumer, Box<dyn std::error::Error>> {
        //TODO: Bind here each event, callback pair.
        for (event, callback) in self.queue_manager.as_ref()
            .ok_or("Error binding event/callback to channel.")? {
            println!("{:?} {:?}", event, callback)
        }
        Ok(Consumer {
            connection: self.connection.ok_or("Unable to set a connection to rabbitMQ.")?,
            queue_manager: self.queue_manager.ok_or("Queue manager not currently active.")?
        })
    }
    //TODO: Add custom error handling on consumer start.
    //TODO: Add channel connection, and exchange binding according to AMPQ-RS API Docs.
}
