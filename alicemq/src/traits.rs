use async_trait::async_trait;

#[async_trait]
pub trait Runner {
    async fn run(&self, _message: String) {}
}
