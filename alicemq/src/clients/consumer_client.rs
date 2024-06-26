use std::future::Future;
use amqprs::callbacks::DefaultChannelCallback;
use amqprs::channel::{BasicConsumeArguments, BasicQosArguments, Channel, QueueBindArguments, QueueDeclareArguments};
use amqprs::connection::{Connection, OpenConnectionArguments};
use tokio::sync::Notify;
use tracing::{debug, error};
use crate::consumers::base_consumer::BaseConsumer;
use crate::settings::configuration::ConnectionSettings;
use serde_json::{Value};

#[allow(dead_code)]
pub struct ConsumerManager {
    connection: Connection,
    channels : Vec<Channel>
}

impl ConsumerManager {
    
    pub fn new_instance() -> ConsumerBuilder {
        ConsumerBuilder {}
    }

    pub async fn run(self, long_lived: bool) {
        if long_lived {
            debug!("Consuming messages from queue.");
            let _guard = Notify::new();
            _guard.notified().await;
        } else {
            for channel in self.channels {
                channel.close().await.unwrap();
            }
            self.connection.close().await.unwrap();
        }
    }

    pub async fn set_queue<F, Fut>(
        &mut self, 
        queue: &'static str, 
        consumer: BaseConsumer<F, Fut>, 
        prefetch: Option<u16>
    )
        where
            F: Fn(Value) -> Fut + Send + 'static,
            Fut: Future<Output = ()> + Send + 'static
    {
        let _channel = self.connection.open_channel(None).await;
        match _channel {
            Ok(_opened_channel) => {
                _opened_channel
                    .register_callback(DefaultChannelCallback)
                    .await
                    .unwrap();
                let (queue_name, _, _) = _opened_channel
                    .queue_declare(QueueDeclareArguments::new(queue))
                    .await.unwrap().unwrap();
                _opened_channel.queue_bind(QueueBindArguments::new(
                    &queue_name,
                    "amq.topic",
                    &queue_name
                )).await.unwrap();
                let args = BasicConsumeArguments::new(&queue_name, &queue_name)
                    .manual_ack(false)
                    .finish();
                _opened_channel
                    .basic_qos(BasicQosArguments::new(
                        0,
                        prefetch.unwrap_or(0),
                        false))
                    .await
                    .unwrap();
                _opened_channel
                    .basic_consume(consumer, args)
                    .await
                    .unwrap();
                self.channels.push(_opened_channel);
            },
            Err(_) => {error!("Unable to register channel {:?}", queue)}
        };
    }
}

pub struct ConsumerBuilder;

impl ConsumerBuilder {

    pub async fn connect(self) -> ConsumerManager {
        let _settings = ConnectionSettings::new();
        let _connection = Connection::open(&OpenConnectionArguments::new(
            &_settings.host,
            _settings.port,
            &_settings.username,
            &_settings.password
        )).await.unwrap();
        ConsumerManager { connection: _connection, channels: vec!() }
    }
}


