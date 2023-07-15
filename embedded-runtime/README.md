[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/embedded-runtime-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/embedded-runtime-rust)
[![docs.rs](https://docs.rs/embedded-runtime/badge.svg)](https://docs.rs/embedded-runtime)
[![crates.io](https://img.shields.io/crates/v/embedded-runtime.svg)](https://crates.io/crates/embedded-runtime)
[![Download numbers](https://img.shields.io/crates/d/embedded-runtime.svg)](https://crates.io/crates/embedded-runtime)
[![dependency status](https://deps.rs/crate/embedded-runtime/latest/status.svg)](https://deps.rs/crate/embedded-runtime)


# `embedded-runtime`
This crate provides a tiny async runtime, targeted at embedded devices. Therefore, it provides a single-threaded
executor as well as a stack-allocated box to box futures.

## Runtime setup
This crate is hardware-independent, and needs some dependency-injection to perform signal/wait on your target hardware.

To provide these dependencies, you need to define the following two `extern "Rust"` functions:
- `_runtime_waitforevent_TBFzxdKN`: Blocks until an event occurs (may wake spuriously). Important: Events must not be
  lost. If an event has occurred between the last invocation and this invocation, this function must not block.
- `_runtime_sendevent_3YSaPmB7`: Raises an event. Important: Events must not be lost. If an event is sent, but the
  receiver is not currently waiting, it must be retained until the receiver tries to wait again.

For an example on how to inject your implementation, see also `embedded-runtime-rp2040`, which is basically this crate
with `wfe`/`sev`-based runtime functions for `rp2040` boards.

## Example
```rust
# use core::{
#     future::Future,
#     pin::Pin,
#     task::{Poll, Context}
# };
#
# /// Blocks until an event occurs (may wake spuriously)
# #[no_mangle]
# #[allow(non_snake_case)]
# pub fn _runtime_waitforevent_TBFzxdKN() {
#     // No-op
# }
# 
# /// Raises an event
# #[no_mangle]
# #[allow(non_snake_case)]
# pub fn _runtime_sendevent_3YSaPmB7() {
#     // No-op
# }
use embedded_runtime::run;

/// A countdown future that resolves to pending until the poll-countdown becomes zero
struct CountdownFuture {
    /// The current countdown value
    countdown: usize
}
impl CountdownFuture {
    /// Creates a new countdown future
    pub const fn new(countdown: usize) -> Self {
        Self { countdown }
    }
}
impl Future for CountdownFuture {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Decrement the value if we are still pending
        if self.countdown > 0 {
            // Print the countdown
            println!("{}!", self.countdown);

            // Decrement the future, wake the executor and return pending
            *self = Self::new(self.countdown - 1);
            cx.waker().wake_by_ref();
            return Poll::Pending;
        }

        // Return ready
        println!("Liftoff 🚀");
        Poll::Ready(())
    }
}

// This creates a new runtime and executes the given futures in an async context
run!(async {
    CountdownFuture::new(3).await;
    CountdownFuture::new(7).await;
}).expect("failed to perform countdowns");
```
