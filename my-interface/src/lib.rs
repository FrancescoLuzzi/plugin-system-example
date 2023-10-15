use async_trait::async_trait;
use pin_project_lite::pin_project;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::runtime::Handle;

#[async_trait]
pub trait SayHelloService {
    async fn say_hello(&self);
}

pin_project! {
    pub struct WithRuntime<F> {
        get_runtime: fn()->Handle,
        #[pin]
        fut: F,
    }
}

/// https://stackoverflow.com/questions/77294605/library-plugin-manager-in-rust-is-it-even-doable-right-now#comment136267977_77295025
impl<F> WithRuntime<F> {
    pub fn new(get_runtime: fn() -> Handle, fut: F) -> Self {
        Self { get_runtime, fut }
    }
}

impl<F> Future for WithRuntime<F>
where
    F: Future,
{
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = (this.get_runtime)().enter();
        this.fut.poll(ctx)
    }
}
