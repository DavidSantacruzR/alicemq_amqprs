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

Define an event queue.
```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let new_event = "my_custom_event".to_string();
    let new_callback = BaseCallback;
    Ok(())
}
```

To set up a basic consumer, connect to a node, set the queue manager
then add the created events, and their handlers.

````rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let new_event = "my_custom_event".to_string();
    let new_callback = BaseCallback;
    let _ = Consumer::new()
        .set_connection_arguments()?
        .connect()
        .await?
        .set_queue_manager()
        .set_event_callback(new_event, new_callback)
        .start_consumer()
        .await?;
    Ok(())
}
````
The following code, will create the queues on a rabbitMQ node, no_ack.

Creating a smart publisher
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

## Running examples

To run any of the examples in the folder, run the following command:

```zsh
cargo run --example my_example
```
