use async_trait::async_trait;
use pin_project_lite::pin_project;

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::runtime::Handle;

/// pass handle to the call, since depending on th runtime using a single static handle could besuboptimal
/// https://stackoverflow.com/questions/77294605/library-plugin-manager-in-rust-is-it-even-doable-right-now#comment136267977_77295025
#[async_trait]
pub trait SayHelloService {
    async fn say_hello(&self, handle: Handle);
}

pin_project! {
    pub struct WithRuntime<F> {
        runtime: Handle,
        #[pin]
        fut: F,
    }
}

impl<F> WithRuntime<F> {
    pub fn new(runtime: Handle, fut: F) -> Self {
        Self { runtime, fut }
    }
}

impl<F> Future for WithRuntime<F>
where
    F: Future,
{
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        let _guard = this.runtime.enter();
        this.fut.poll(ctx)
    }
}
