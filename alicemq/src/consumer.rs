use std::collections::HashMap;
use amqprs::connection::{Connection};
use std::default::Default;
use std::error::Error;
use amqprs::callbacks::{DefaultChannelCallback, DefaultConnectionCallback};
use amqprs::channel::{BasicConsumeArguments, Channel, QueueBindArguments, QueueDeclareArguments};
use tracing_subscriber::FmtSubscriber;
use tracing::{info, Level};

use crate::constants::{ROUTING_KEY, EXCHANGE_NAME};
use crate::{callback::*, connection_arguments::*};

/// The consumer allows for the creation of a smart consumer, adding as many events,
/// and custom message handlers as necessary.
/// # Example
/// ```rust
/// use alicemq::consumer::{Consumer};
/// use alicemq::callback::{BaseCallbackConsumer};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let new_event = "test_event".to_string();
///     let new_callback = BaseCallbackConsumer::new(false);
///     let mut consumer = Consumer::new()
///         .connect()
///         .await?
///         .set_queue_manager()
///         .build()
///         .unwrap()
///         .set_event_callback(new_event, new_callback);
///
///     consumer
///         .start_consumer()
///         .await?;
///     Ok(())
/// }
/// ```

pub struct Consumer {
    /// The consumer needs a vector of registered channels
    /// to avoid dropping bindings to a specific queue.
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
    /// The builder main point is hold the rabbitMQ connection,
    /// and the queue manager to pass it to a Consumer Instance when it is created.
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
    /// set event callback push to the queue manager each event / callback pair.
    /// Each of the callbacks should be of type of an BaseCallbackConsumer.
    /// You can implement either an AsyncConsumer or any of the provided by the amqprs library.
    pub fn set_event_callback(mut self, event_queue: String, callback: BaseCallbackConsumer) -> Self {
        let _ = &self.queue_manager.insert(
            event_queue,
            callback
        );
        self
    }
    /// There is no public method for this type to stop consuming on a given time.
    /// Therefore, the consumer must be closed to drop all connections.
    pub async fn start_consumer(&mut self) -> Result<(), Box<dyn Error>> {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        let queue_manager = self.queue_manager.clone();
        for (event, callback_handler) in queue_manager {
            let channel = self.connection
                .open_channel(None)
                .await
                .unwrap();
            channel
                .register_callback(DefaultChannelCallback)
                .await
                .unwrap();
            let (queue_name, _, _) = channel
                .queue_declare(QueueDeclareArguments::new(&event))
                .await
                .unwrap()
                .unwrap();
            let routing_key = ROUTING_KEY;
            let exchange_name = EXCHANGE_NAME;
            channel
                .queue_bind(QueueBindArguments::new(
                    &queue_name,
                    exchange_name,
                    routing_key,
                ))
                .await
                .unwrap();
            let args = BasicConsumeArguments::new(
                &queue_name,
                "basic_consumer"
            )
                .no_ack(false)
                .finish();
            channel
                .basic_consume(callback_handler, args)
                .await
                .unwrap();
            let _ = &self.registered_channels.push(channel);
            info!("Started consumer ...");
        }
        Ok(())
    }
}
