use std::collections::HashMap;
use amqprs::connection::{Connection, OpenConnectionArguments};
use std::default::Default;
use std::error::Error;
use amqprs::callbacks::DefaultConnectionCallback;
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
    event_queue: String,
    connection: Connection,
    queue_manager: Vec<HashMap<String, BaseCallback>>
}

impl Consumer {
    pub fn new() -> ConsumerBuilder {
        ConsumerBuilder::default()
    }
}

#[derive(Default)]
pub struct ConsumerBuilder {
    event_queue: Option<String>,
    connection: Option<Connection>,
    connection_arguments: Option<ConnectionArguments>,
    queue_manager: Option<Vec<HashMap<String, BaseCallback>>>
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
    pub fn set_event_queue(mut self, queue_reference: String) -> Result<Self, Box<dyn std::error::Error>> {
        self.event_queue.get_or_insert(queue_reference);
        Ok(self)
    }
    pub async fn connect(mut self) -> Result<Self, Box<dyn std::error::Error>> {
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
}
