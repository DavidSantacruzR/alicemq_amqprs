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

### Implementing a custom consumer handler.

Currently, non-blocking async consumers, are supported. To define a new AsyncConsumer
first implement a custom callback. indicate if it should or should not handle a custom ack.

```rust
struct ConsumerCallback {
    no_ack: bool
}
````

Defining the AsyncConsumer from the AMPQRS library

```rust
use async_trait::async_trait;

#[async_trait]
impl AsyncConsumer for ConsumerCallback {
    async fn consume(
        &mut self,
        channel: &Channel,
        deliver: Deliver,
        _basic_properties: BasicProperties,
        _content: Vec<u8>,
    ) {
        info!("got message with data {}", std::str::from_utf8(&_content).unwrap());
        if !self.no_ack {
            let args = BasicAckArguments::new(deliver.delivery_tag(), false);
            channel.basic_ack(args).await.unwrap();
        }
    }
}
````

To create a consumer, use the consumer manager, and define if it's long-lived.
Define a queue and its respective callback handler.

```rust
#[tokio::main]
async fn main() {

    let queue: String = "test_event".to_string();

    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .add_channel()
        .await
        .build();

    test_consumer
        .set_event_queue(
            queue,
            ConsumerCallback {no_ack: false}
        ).await
        .run(true).await;
}
```

To create a smart publisher simply create an instance of a smart publisher. 
Supported data to deliver, strings only.

### Creating a smart publisher
```rust
use alicemq::{publisher::Publisher};
use tokio;

#[tokio::main]
async fn main() {

    let publisher = Publisher {};
    let message = String::from("data: {field_1: some data}");
    publisher.send_message(message, "test_event".to_string()).await;
}
```

## Running examples

To run any of the examples in the folder, run the following command:

```zsh
cargo run --example my_example
```
