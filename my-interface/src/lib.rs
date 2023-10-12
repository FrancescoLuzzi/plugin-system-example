use async_trait::async_trait;
#[async_trait]
pub trait SayHelloService {
    async fn say_hello(&self);
}
