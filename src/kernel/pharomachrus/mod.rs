use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::kernel::error::KernelResult;

pub struct Pharomachrus {}

impl Pharomachrus {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn execute<F, T>(&self, future: F) -> KernelResult<T>
    where
        F: Future<Output = KernelResult<T>>,
    {
        future.await
    }

    pub fn poll_future<F, T>(
        &self,
        mut future: Pin<&mut F>,
        cx: &mut Context<'_>,
    ) -> Poll<KernelResult<T>>
    where
        F: Future<Output = KernelResult<T>>,
    {
        future.as_mut().poll(cx)
    }
}

impl Default for Pharomachrus {
    fn default() -> Self {
        Self::new()
    }
}
