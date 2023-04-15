use amqprs::connection::{Connection, OpenConnectionArguments};
use crate::settings::base::{Config};

struct ConsumerManager;

#[derive(Default)]
struct ConsumerBuilder {
    config: Config,
    connection: Option<Connection>,
}

impl ConsumerManager {
    fn new() -> ConsumerBuilder {
        ConsumerBuilder::default()
    }
}

impl ConsumerBuilder {
    fn load_settings(&mut self) -> Config {
        Config::new()
    }

    async fn connect(&mut self) {
        let config = self.load_settings();
        let connection = Connection::open(&OpenConnectionArguments::new(
            &config.host,
            config.port,
            &config.username,
            &config.password
        ))
            .await
            .unwrap();
        self.connection = Some(connection);
    }
}