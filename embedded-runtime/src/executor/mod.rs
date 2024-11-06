//! The executor for this runtime

mod waker;

use crate::{err, error::Error, runtime};
use core::{future::Future, pin::Pin, task::Context};

/// A tiny stack-based, single-threaded async executor suitable for embedded runtimes
///
/// # Generics
/// - `'a`: Tied to the lifetime of the pinned futures to execute (i.e. this executor must not outlive the futures it
///   executes)
/// - `T`: The type of futures to execute (e.g. `dyn Future<Output = ()>`)
/// - `LEN`: The maximum amount of top-level futures this executor can execute (defaults to `32`)
pub struct Executor<'a, T, const LEN: usize = 32>
where
    T: ?Sized,
{
    /// The registered futures
    futures: [Option<Pin<&'a mut T>>; LEN],
    /// The current length of `futures`
    len: usize,
}
impl<'a, T, const LEN: usize> Executor<'a, T, LEN>
where
    T: Future<Output = ()> + ?Sized,
{
    /// Initialization value for const initializer
    const INIT: Option<Pin<&'a mut T>> = None;

    /// Creates a new executor
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { futures: [Self::INIT; LEN], len: 0 }
    }

    /// Registers a new future for execution
    pub fn register(&mut self, future: Pin<&'a mut T>) -> Result<&mut Self, Error> {
        // Get the next free slot
        let Some(slot) = self.futures.get_mut(self.len) else {
            return Err(err!("Executor has not enough space to register more futures"));
        };

        // Store the future
        *slot = Some(future);
        self.len += 1;
        Ok(self)
    }

    /// Runs the executor
    pub fn run(&mut self) {
        // Create waker and context
        let waker = waker::new();
        let mut context = Context::from_waker(&waker);

        // Repeatedly poll all futures until they have completed
        let mut remaining = self.len;
        while remaining > 0 {
            // Poll each future
            'poll_loop: for index in 0..self.len {
                // Get the future if any
                let Some(future) = &mut self.futures[index] else {
                    continue 'poll_loop;
                };

                // Poll the future
                if future.as_mut().poll(&mut context).is_ready() {
                    // Unregister the future if it is dome
                    self.futures[index] = None;
                    remaining -= 1;
                }
            }

            // Wait until at least one future signales that it is ready
            // Note: We do this *after* the polling to ensure that each future is at least polled once
            unsafe { runtime::_runtime_waitforevent_TBFzxdKN() };
        }
    }
}

/// Creates an executor and executes the given futures
#[macro_export]
macro_rules! run {
    ($($futures:expr),+) => {{
        /// Executes the given futures
        let execute = || -> core::result::Result<(), $crate::error::Error> {
            // Create executor
            let mut executor: $crate::executor::Executor<'_, dyn core::future::Future<Output = ()>> =
                $crate::executor::Executor::new();

            // Register futures
            $(
                let future = core::pin::pin!($futures);
                executor.register(future as core::pin::Pin<&mut dyn core::future::Future<Output = ()>>)?;
            )*

            // Execute the futures
            executor.run();
            Ok(())
        };

        // Execute all futures
        execute()
    }};
}
