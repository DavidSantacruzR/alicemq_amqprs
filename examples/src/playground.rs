
use amqprs::{connection::{Connection, OpenConnectionArguments},
             callbacks::{DefaultChannelCallback, DefaultConnectionCallback}};

#[tokio::main]
async fn main() {
    let connection = Connection::open(&OpenConnectionArguments::new(
        "localhost",
        5672,
        "guest",
        "guest",
    ))
        .await
        .unwrap();
    connection
        .register_callback(DefaultConnectionCallback)
        .await
        .unwrap();
}