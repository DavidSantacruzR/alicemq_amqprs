# Alicemq_amqprs
### This is a simple implementation of a wrapper / adapter to the official AMQPRS library

Checkout the project: https://github.com/gftea/amqprs

**_This is a fun side project for learning, and implement my current knowledge in Rustlang._**

To use the library you have to first create the event queues, and their specific handlers.
Every handler must a function that receives the type ```String``` only.

## Running RabbitMQ Locally

To run a local instance of RabbitMQ, use the following command:

```zsh
rabbitmq-server
```

To create a smart-publisher subscriber start by importing the required types.
```rust
use std::future::Future;
use tracing::{debug, Level};
use alicemq::clients::consumer_client::ConsumerManager;
use alicemq::consumers::base_consumer::BaseConsumer;
use tracing_subscriber::FmtSubscriber;
```

### Implementing a custom consumer handler.

Currently, non-blocking async consumers, are supported. To define a handler follow 
the subsequent structure:

```rust
async fn my_callback(data: Vec<u8>) -> impl Future<Output = ()> {
    async {
        debug!("Received data: {:?}", String::from_utf8(data));
    }
}
````

### Creating a smart long-lived consumer

To create a consumer, use the consumer manager, and define if it's long-lived.
Define a queue and its respective callback handler.

```rust
#[tokio::main]
async fn main() {
    let _ = set_tracing_subscriber();
    let mut _manager: ConsumerManager = ConsumerManager::new_instance()
        .connect().await;
    _manager.set_queue("test_queue", BaseConsumer::new(
        |data| {
            async move {
                my_callback(data).await.await;
            }
        },
    ), None).await;
    _manager.set_queue("another_test_queue", BaseConsumer::new(
        |data| {
            async move {
                my_callback(data).await.await;
            }
        },
    ), None).await;
    _manager.run(true).await;
}
```

To create a smart publisher simply create an instance of a smart publisher. 
Supported data to deliver, strings only.

### Creating a smart publisher
```rust
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use alicemq::clients::publisher_client::Publisher;

fn set_tracing_subscriber() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}

#[tokio::main]
async fn main() {
    let _ = set_tracing_subscriber();
    for i in 1..10 {
        let _ = Publisher::send_message(
            format!("This message contains the id {}", i), String::from("test_queue")
        ).await;
    }
}
```

## Running examples

To run any of the examples in the folder, run the following command:

```zsh
cargo run --example my_example
```
