//! Futures to interrupt the current async context to give timeslices to other pending tasks

use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// A future that can be polled once before it becomes ready; useful to cooperatively give up a timeslice to the
/// runtime/other pending futures
///
/// # Behaviour
/// Polling this future will immediately wake the waker again and yield, making room for other futures to execute. This
/// is useful for e.g. running intensive loops or similar inside a future.
#[derive(Debug, Default)]
pub struct SpinFuture {
    /// Whether the future has been polled already or not
    polled: bool,
}
impl SpinFuture {
    /// Creates a new spin future
    ///
    /// # Note
    /// This future should usually not be constructed directly, use [`spin_once`] instead.
    pub const fn new() -> Self {
        Self { polled: false }
    }
}
impl Future for SpinFuture {
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

/// A passive future that can be polled once before it becomes ready; useful to suspend the current context and up a
/// timeslice to the runtime/other pending futures
///
/// # Behaviour
/// Polling this future yields immediately **WITHOUT WAKING THE WAKER AGAIN**, making room for other futures to execute.
/// This is useful to e.g. suspend the execution of the current routine if it cannot make any progress and has no
/// associated event sources.
///
/// # Important
/// Unlike [`SpinFuture`], this future **DOES NOT** wake the waker again. If the runtime enters deep-sleep, you **MUST**
/// ensure there are other pending futures or event sources to wake it up again.
#[derive(Debug, Default)]
pub struct SleepFuture {
    /// Whether the future has been polled already or not
    polled: bool,
}
impl SleepFuture {
    /// Creates a new spin future
    ///
    /// # Note
    /// This future should usually not be constructed directly, use [`sleep_once`] instead.
    pub const fn new() -> Self {
        Self { polled: false }
    }
}
impl Future for SleepFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.polled {
            // Mark the future as polled so that it returns ready the next time
            self.polled = true;
            Poll::Pending
        } else {
            // The future has been polled, so it's ready now
            Poll::Ready(())
        }
    }
}

/// A function that can be awaited once before it returns; useful to cooperatively give up a timeslice to the
/// runtime/other pending futures
///
/// # Behaviour
/// Awaiting this function will immediately wake the waker again and yield, making room for other futures to execute.
/// This is useful for e.g. running intensive loops or similar inside a future.
pub async fn spin_once() {
    SpinFuture::new().await
}

/// A function that can be polled once before it becomes ready; useful to suspend the current context and up a timeslice
/// to the runtime/other pending futures
///
/// # Behaviour
/// Awaiting this function yields immediately **WITHOUT WAKING THE WAKER AGAIN**, making room for other futures to
/// execute. This is useful to e.g. suspend the execution of the current routine if it cannot make any progress and has
/// no associated event sources.
///
/// # Important
/// Unlike [`spin_once`], this function **DOES NOT** wake the waker again. If the runtime enters deep-sleep, you
/// **MUST** ensure there are other pending futures or event sources to wake it up again.
pub async fn sleep_once() {
    SleepFuture::new().await
}
