use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A future that must be polled once before it becomes ready; useful to cooperatively give up a timeslice to the
/// runtime/other pending futures
///
/// Calling this function will move the currently executing future to the back of the execution queue, making room for
/// other futures to execute. This is especially useful after running CPU-intensive operations inside a future.
#[derive(Debug)]
pub struct YieldFuture {
    /// Whether the future has been polled already or not
    polled: bool,
}
impl YieldFuture {
    /// Creates a new yielding future
    ///
    /// # Note
    /// This future should usually not be constructed directly, use `yield_now` instead.
    pub const fn new() -> Self {
        Self { polled: false }
    }
}
impl Future for YieldFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if !self.polled {
            // Mark the future as polled so that it returns ready the next time
            self.polled = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            // The future has been polled, so it's ready now
            Poll::Ready(())
        }
    }
}

/// Cooperatively gives up a timeslice to the runtime/other pending futures
///
/// Calling this function will move the currently executing future to the back of the execution queue, making room for
/// other futures to execute. This is especially useful after running CPU-intensive operations inside a future.
pub async fn yield_now() {
    YieldFuture::new().await
}
