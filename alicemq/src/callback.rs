
#[derive(Debug, Clone, Copy)]
pub struct BaseCallbackConsumer {
    pub ack: bool
}

impl BaseCallbackConsumer {
    pub async fn handle(self, message: String) {
        println!("received message: {}", message);
    }
}
