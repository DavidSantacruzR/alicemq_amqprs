
/* Enum runtime to define if execution is async or in a blocking context.*/
#[derive(PartialEq)]
enum Runtimes {
    ASYNCHRONOUS,
    SYNCHRONOUS
}

struct SomeCustomStruct {
    message: String
}

impl SomeCustomStruct {
    pub fn handle(&self) {
        println!("The message is being handled, and passed down to the custom struct {}", self.message);
    }
}

// Trait that is in charge of executing the desired code handler.
pub trait Runner {
    fn run(&self, _message: String) {}
}

// Base callback for which any executor must run inside the run method, this must be strictly
// implemented.
pub struct BaseCallback {
    runtime: Runtimes
}

//Callback runner must be implemented inside the channel and implement either an async or
// a blocking consumer with it's respective callback handler.
pub struct CallbackRunner;

// In the implementation we can use as many functions as we need, same with structs.
impl Runner for BaseCallback {
    fn run(&self, message: String) {
        println!("Got a message, and I'm handling it ... {}", message);
        println!("creating a custom struct");
        let handler = SomeCustomStruct { message };
        handler.handle();
    }
}

impl CallbackRunner {
    pub fn run_callbacks(&self, data: String, callback: BaseCallback) {
        match callback.runtime {
            Runtimes::ASYNCHRONOUS => {
                println!("Running in an async context.");
                callback.run(data);
            },
            Runtimes::SYNCHRONOUS => {
                println!("Running in a blocking context.");
                callback.run(data);
            },
        };
    }
}

fn main() {

    let runner = CallbackRunner;
    let base_callback = BaseCallback {runtime: Runtimes::SYNCHRONOUS};

    let base_async_callback = BaseCallback {runtime: Runtimes::ASYNCHRONOUS};

    runner.run_callbacks("Sync message ... working".to_string(), base_callback);
    runner.run_callbacks("Async message ... + ... working ".to_string(), base_async_callback);

}