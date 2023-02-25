
#[derive(Debug)]
pub struct BaseCallback;

pub trait HandleMessage {
    fn handle_message(&self) -> String;
}

impl HandleMessage for BaseCallback {
    fn handle_message(&self) -> String {
        format!("Handling...")
    }
}
