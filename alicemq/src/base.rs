use async_trait::async_trait;
use crate::enums::Runtime;
use crate::traits::Runner;

#[derive(Clone)]
pub struct BaseCallback {
    pub runtime: Runtime
}

pub struct CallbackRunner;

#[async_trait]
impl Runner for BaseCallback {
    async fn run(&self, _message: String) {
        println!("Running callback.");
    }
}

impl CallbackRunner {
    pub fn run_callbacks(&self, data: String, callback: BaseCallback) {
        match callback.runtime {
            Runtime::ASYNCHRONOUS => {
                println!("Running in an async context.");
                let _ = callback.run(data);
            },
            Runtime::SYNCHRONOUS => {
                println!("Running in a blocking context.");
                let _ = callback.run(data);
            },
        };
    }
}