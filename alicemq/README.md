# Alicemq_amqprs
### This is a simple implementation of a wrapper / adapter to the official AMQPRS library

Checkout the project: https://github.com/gftea/amqprs

To use the library you have to first create the event queues, and their specific handlers.
Events are expected to be of type string, and handlers implementations of BaseCallbacks.

## Running RabbitMQ Locally

To run a local instance of RabbitMQ, use the following command:

```zsh
rabbitmq-server
```

To create a smart-publisher subscriber start by importing the required types.
```rust
use tokio;
use alicemq::consumer::{Consumer};
use alicemq::callback::{BaseCallback};
```

### Defining an event queue.

The BaseCallback defines a parameter to determine if manual ack should be implemented.

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let new_event = "my_custom_event".to_string();
    let new_callback = BaseCallbackConsumer::new(false);
    Ok(())
}
```

To set up a basic consumer, connect to a node, set the queue manager
then add the created events, and their handlers.

````rust
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: Add tracing for publishing messages instead of prints.
    let new_event = "test_event".to_string();
    let new_callback = BaseCallbackConsumer::new(false);
    let consumer = Consumer::new()
        .connect()
        .await?
        .set_queue_manager()
        .build()
        .unwrap()
        .set_event_callback(new_event, new_callback);
    
    //Have the consumer running and in scope, otherwise It'll drop active connections.
    consumer
        .start_consumer()
        .await?;
        
}
````
The following code, will create the queues on a rabbitMQ node, no_ack.

### Creating a smart publisher
```rust
use alicemq::publisher::Publisher;

#[tokio::main]
async fn main() {
    let _ = Publisher::new()
        .connect()
        .await.unwrap()
        .build()
        .unwrap()
        .send_message("test_event".to_string(), data.to_string()).await;
}
```

### Implementing a custom consumer handler.

To handle all the incoming messages in a specific manner, you can do an implementation
of the ```AsyncConsumer```, and pass it as an argument when setting an event callback.

```rust
use std::str;
use amqprs::channel::{BasicAckArguments, Channel};
use amqprs::consumer::AsyncConsumer;
use amqprs::{BasicProperties, Deliver};
use async_trait::async_trait;
use tracing::info;

#[derive(Debug, Clone, Copy)]
pub struct BaseCallbackConsumer {
    pub no_ack: bool
}

impl BaseCallbackConsumer {
    pub fn new(no_ack: bool) -> Self {
        Self { no_ack }
    }
}

#[async_trait]
impl AsyncConsumer for BaseCallbackConsumer {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        _content: Vec<u8>,
    ) {
        //You can define here your desired behaviour.
        
        info!("got message {:?}", std::str::from_utf8(&_content));
        if !self.no_ack {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}

```

## Running examples

To run any of the examples in the folder, run the following command:

```zsh
cargo run --example my_example
```
