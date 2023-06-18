//! The waker

use crate::runtime;
use core::{
    ptr,
    task::{RawWaker, RawWakerVTable, Waker},
};

/// The VTable for the raw waker
static VTABLE: RawWakerVTable = RawWakerVTable::new(impl_clone, impl_wake, impl_wake_by_ref, impl_drop);

/// This function will be called when the `RawWaker` gets cloned, e.g. when the `Waker` in which the `RawWaker` is stored
/// gets cloned.
///
/// The implementation of this function must retain all resources that are required for this additional instance of a
/// `RawWaker` and associated task. Calling `wake` on the resulting `RawWaker` should result in a wakeup of the same task
/// that would have been awoken by the original `RawWaker`.
fn impl_clone(_state: *const ()) -> RawWaker {
    RawWaker::new(ptr::null(), &VTABLE)
}

/// This function will be called when `wake` is called on the `Waker`. It must wake up the task associated with this
/// `RawWaker`.
///
/// The implementation of this function must make sure to release any resources that are associated with this instance of a
/// `RawWaker` and associated task.
fn impl_wake(_state: *const ()) {
    unsafe { runtime::_runtime_sendevent_3YSaPmB7() };
}

/// This function will be called when `wake_by_ref` is called on the `Waker`. It must wake up the task associated with this
/// `RawWaker`.
///
/// This function is similar to `wake`, but must not consume the provided data pointer.
fn impl_wake_by_ref(_state: *const ()) {
    unsafe { runtime::_runtime_sendevent_3YSaPmB7() };
}

/// This function gets called when a `Waker` gets dropped.
///
/// The implementation of this function must make sure to release any resources that are associated with this instance of a
/// `RawWaker` and associated task.
fn impl_drop(_state: *const ()) {
    // Do nothing
}

/// Creates a new waker
pub fn new() -> Waker {
    let raw_waker = RawWaker::new(ptr::null(), &VTABLE);
    unsafe { Waker::from_raw(raw_waker) }
}
