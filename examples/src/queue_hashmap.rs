use std::collections::HashMap;

pub struct QueueManager {
    event_queue: HashMap<String, String>
}

//Creates a new queue manager with an empty queue.
impl QueueManager {
    pub fn new() -> QueueBuilder {
        QueueBuilder::default()
    }
}

//For the queue builder set a default implementation, when creating a new one.
//By default it'll set an empty HashMap.
#[derive(Default, Debug)]
pub struct QueueBuilder {
    event_queue: HashMap<String, String>
}


impl QueueBuilder {
    pub fn set_new_event(mut self, event:String, callback:String)
        -> Result<Self, Box<dyn std::error::Error>> {
        self.event_queue.insert(
            event,
            callback
        );
        Ok(self)
    }
}

fn main() {
    let queue_manager = QueueManager::new();
    print!("{:?}", queue_manager);
}
