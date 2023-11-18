
/* Enum runtime to define if execution is async or in a blocking context.*/
#[derive(PartialEq)]
enum Runtimes {
    ASYNCHRONOUS,
    SYNCHRONOUS
}

// Trait that is in charge of executing the desired code handler.
pub trait Runner {
    fn run(&self, message: String) {
        println!("{}", message);
    }
}

// Base callback for which any executor must run inside the run method, this must be strictly
// implemented.
pub struct BaseCallback {
    runtime: Runtimes
}

pub struct CallbackRunner;

impl Runner for BaseCallback {
    fn run(&self, message: String) {
        println!("Got a message, and I'm handling it ... {}", message);
    }
}

impl CallbackRunner {
    pub fn run_callbacks(&self, data: String, callback: BaseCallback) {
        if callback.runtime == Runtimes::ASYNCHRONOUS {
            println!("Running in an async context.");
            callback.run(data);
    } else {
            println!("Running in a blocking context.");
            callback.run(data);
        }
    }
}

fn main() {

    let runner = CallbackRunner;
    let base_callback = BaseCallback {runtime: Runtimes::SYNCHRONOUS};

    let base_async_callback = BaseCallback {runtime: Runtimes::ASYNCHRONOUS};

    runner.run_callbacks("Sync message ... working".to_string(), base_callback);
    runner.run_callbacks("Async message ... + ... working ".to_string(), base_async_callback);

}