use amqprs::connection::{Connection, OpenConnectionArguments};
use dotenv;
use std::default::Default;

#[derive(Default)]
pub struct ConnectionArguments;

impl Default for ConnectionArguments {
    fn default() -> OpenConnectionArguments {
        OpenConnectionArguments::new(
            "localhost",
            5672,
            "guest",
            "guest",
        )
    }
}

pub struct Consumer {
    event_queue: String,
    connection: Connection,
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
    connection_arguments: Option<ConnectionArguments>
}

impl ConsumerBuilder {
    pub fn set_connection_arguments(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();
        let host: String = std::env::var("host").expect("No host set.");
        let port: String = std::env::var("host").expect("No port set.");
        let username: String = std::env::var("host").expect("No user set.");
        let password: String = std::env::var("host").expect("No pwd set.");
        self.connection_arguments = OpenConnectionArguments::new(
            &host,
            port.parse::<u16>()?,
            &username,
            &password
        )?;
        Ok(self)
    }
}
