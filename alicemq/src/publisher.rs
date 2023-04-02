use amqprs::BasicProperties;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicPublishArguments, QueueBindArguments};
use amqprs::connection::{Connection};
use tokio::time;
use crate::connection_arguments::ConnectionArguments;
use crate::constants::{ROUTING_KEY, EXCHANGE_NAME};

#[derive(Clone)]
pub struct Publisher {
    connection: Connection
}

impl Publisher {
    pub fn new() -> PublisherBuilder {
        PublisherBuilder::default()
    }
    pub async fn send_message(self, queue: String, data: String) {
        let delivered_content = data.clone().into_bytes();
        let publishing_args = BasicPublishArguments::new(EXCHANGE_NAME, ROUTING_KEY);
        let channel = self.connection.open_channel(None).await.unwrap();
        channel
            .queue_bind(QueueBindArguments::new(
                &queue,
                EXCHANGE_NAME,
                ROUTING_KEY
            ))
            .await
            .unwrap();
        channel
            .register_callback(DefaultChannelCallback)
            .await
            .unwrap();
        channel
            .basic_publish(BasicProperties::default(), delivered_content, publishing_args)
            .await
            .unwrap();
        time::sleep(time::Duration::from_millis(100)).await;
        //TODO: Add tracing to sending messages to the consumer.
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
