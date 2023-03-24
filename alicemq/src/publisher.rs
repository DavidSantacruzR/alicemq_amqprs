use amqprs::BasicProperties;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::BasicPublishArguments;
use amqprs::connection::{Connection};
use tokio::time;
use crate::connection_arguments::ConnectionArguments;
use crate::constants::{ROUTING_KEY};

pub struct Publisher {
    connection: Connection
}

impl Publisher {
    pub fn new() -> PublisherBuilder {
        PublisherBuilder::default()
    }
    pub async fn send_message(mut self, queue: String, data: String) {
        let deliver_content = data.clone().into_bytes();
        let publishing_args = BasicPublishArguments::new(&queue, ROUTING_KEY);
        let channel = self.connection.open_channel(None).await.unwrap();
        channel
            .register_callback(DefaultChannelCallback)
            .await
            .unwrap();
        channel
            .basic_publish(BasicProperties::default(), deliver_content, publishing_args)
            .await
            .unwrap();
        time::sleep(time::Duration::from_secs(1)).await;
        println!("message sent with data: {:?}", data);
    }
}

#[derive(Default)]
pub struct PublisherBuilder {
    connection: Option<Connection>
}

impl PublisherBuilder {
    pub async fn connect(mut self) -> Result<Self, Box<dyn std::error::Error>> {
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

    pub fn build(self) -> Result<Publisher, Box<dyn std::error::Error>> {
        Ok(Publisher {
            connection: self.connection.ok_or("Unable to set a connection to rabbitMQ.")?
        })
    }
}
