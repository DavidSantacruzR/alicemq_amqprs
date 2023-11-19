
pub struct CallbackRunner;

impl CallbackRunner {
    pub async fn run_callbacks<F>(&self, data: String, callback: F) where F: Fn(String) + Send + Copy + 'static {
        callback(data);
    }
}