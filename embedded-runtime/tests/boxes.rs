#![allow(deprecated)]

use embedded_runtime::FutureBox;
use std::{
    future::Future,
    pin::{pin, Pin},
    ptr,
    sync::atomic::{AtomicUsize, Ordering::SeqCst},
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

/// A test future
struct TestFuture<'a> {
    /// A counter that gets increased if the future is polled (for external observability)
    polled: &'a AtomicUsize,
    /// A counter that gets increased if the future is dropped (for external observability)
    dropped: &'a AtomicUsize,
}
impl<'a> TestFuture<'a> {
    /// Creates a new test future
    pub const fn new(polled: &'a AtomicUsize, dropped: &'a AtomicUsize) -> Self {
        Self { polled, dropped }
    }
}
impl<'a> Drop for TestFuture<'a> {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, SeqCst);
    }
}
impl<'a> Future for TestFuture<'a> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        self.polled.fetch_add(1, SeqCst);
        Poll::Pending
    }
}

/// Constructs a dummy waker
fn dummy_waker() -> Waker {
    // Define dummy VTable
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);
    fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(ptr::null(), &VTABLE)
    }
    fn wake(_: *const ()) {
        // Do nothing
    }
    fn wake_by_ref(_: *const ()) {
        // Do nothing
    }
    fn drop(_: *const ()) {
        // Do nothing
    }

    // Create the wakers
    let raw = RawWaker::new(ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw) }
}

#[test]
fn test() {
    // Create the observation vars
    static POLLED: AtomicUsize = AtomicUsize::new(0);
    static DROPPED: AtomicUsize = AtomicUsize::new(0);

    // A scope to ensure the future gets dropped again
    {
        // Box the future
        let future = TestFuture::new(&POLLED, &DROPPED);
        let boxed: FutureBox = FutureBox::new(future).expect("failed to box future");

        // Create a polling context and pin the future
        let waker = dummy_waker();
        let mut context = Context::from_waker(&waker);
        let mut pinned = pin!(boxed);

        // Poll the future
        for expected in 1..=7 {
            let state = pinned.as_mut().poll(&mut context);
            assert!(state.is_pending(), "invalid future state");
            assert_eq!(POLLED.load(SeqCst), expected, "invalid poll count")
        }
    }

    // Ensure the future has been dropped
    assert_eq!(DROPPED.load(SeqCst), 1, "invalid drop count");
}
