# Alicemq_amqprs
### This is a simple implementation of a wrapper / adapter to the official AMQPRS library

Checkout the project: https://github.com/gftea/amqprs

This is a fun side project for learning, and implement my current knowledge in Rustlang.

````
THIS PROJECT IS GETTING A SECOND RE-WRITTE :D
````

To use the library you have to first create the event queues, and their specific handlers.
Every handler must a function that receives the type ```String``` only.

## Running RabbitMQ Locally

To run a local instance of RabbitMQ, use the following command:

```zsh
rabbitmq-server
```

To create a smart-publisher subscriber start by importing the required types.
```rust
use tokio;
use tracing::{Level};
use tracing_subscriber::FmtSubscriber;
use uuid::Uuid;
use alicemq::consumer::ConsumerManager;
```

### Implementing a custom consumer handler.

Currently, non-blocking async consumers, are supported. To define a handler follow 
the subsequent structure:

```rust
fn print_some_stuff(message: String) {
    let id = Uuid::new_v4();
    println!("Got message: {}, stored with uuid: {}", message, id);
}
````

### Creating a smart long-lived consumer

To create a consumer, use the consumer manager, and define if it's long-lived.
Define a queue and its respective callback handler.

```rust
#[tokio::main]
async fn main() {

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let queue: String = "test_event".to_string();

    let test_consumer = ConsumerManager::new()
        .connect()
        .await
        .build();

    test_consumer
        .set_event_queue(
            queue,
            print_some_stuff
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
