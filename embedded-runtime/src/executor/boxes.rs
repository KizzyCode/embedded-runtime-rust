//! A stack-allocated box for futures
#![deprecated = "A lot of unsafe code that is not necessary (use the `run!`-macro instead)"]

use crate::{err, error::Error};
use core::{
    future::Future,
    marker::{PhantomData, PhantomPinned},
    mem,
    pin::Pin,
    ptr,
    task::{Context, Poll},
};

/// A stack-allocated box for futures
#[repr(C, align(64))]
pub struct FutureBox<Output = (), const SIZE: usize = 64> {
    /// The storage for the future
    storage: [u8; SIZE],
    /// The type specific function to call `Future::poll` on the original type
    impl_poll: unsafe fn(self_: Pin<&mut Self>, context: &mut Context) -> Poll<Output>,
    /// The type specific function to drop the value
    impl_drop: unsafe fn(self_: &mut Self),
    /// The future's output type
    _output: PhantomData<Output>,
    /// Ensure that this type cannot be unpinned
    _pinned: PhantomPinned,
}
impl<Output, const SIZE: usize> FutureBox<Output, SIZE> {
    /// Boxes the given future
    pub fn new<T>(future: T) -> Result<Self, Error>
    where
        T: Future<Output = Output> + 'static,
    {
        // Validate type size and alignment
        if mem::size_of::<T>() > SIZE {
            return Err(err!("Future is too large"));
        }
        if mem::align_of::<T>() > mem::align_of::<Self>() {
            return Err(err!("Future has an unsupported alignment"));
        }

        // Move the future into the storage
        let mut storage = [0; SIZE];
        let future_ptr = ptr::addr_of!(future) as *const u8;
        unsafe { storage.as_mut_ptr().copy_from_nonoverlapping(future_ptr, mem::size_of_val(&future)) };

        // Forget future and init self
        mem::forget(future);
        Ok(Self {
            storage,
            impl_poll: Self::impl_poll::<T>,
            impl_drop: Self::impl_drop::<T>,
            _output: PhantomData,
            _pinned: PhantomPinned,
        })
    }

    /// The type specific function to call `Future::poll` on the original type
    unsafe fn impl_poll<T>(self: Pin<&mut Self>, context: &mut Context) -> Poll<Output>
    where
        T: Future<Output = Output>,
    {
        // Get the future pointer
        // Safety: This should be safe since `self` is pinned, and we ensure to not move the contents of `self.storage`. We
        // just cast it back in-place to the original type and pin the resulting reference immediately
        let this = unsafe { self.get_unchecked_mut() };
        let future_ptr = this.storage.as_mut_ptr() as *mut T;
        let future = unsafe { future_ptr.as_mut() }.expect("future pointer is NULL");

        // Create a pinned future
        let future = Pin::new_unchecked(future);
        future.poll(context)
    }

    /// The type specific function to drop the value
    unsafe fn impl_drop<T>(&mut self)
    where
        T: Future<Output = Output>,
    {
        // Get the future pointer and drop it in place
        // Safety: This should be safe because we do not move the contents of `self.storage`, even during drop
        let future_ptr = self.storage.as_mut_ptr() as *mut T;
        ptr::drop_in_place(future_ptr);
    }
}
impl<Output, const SIZE: usize> Drop for FutureBox<Output, SIZE> {
    fn drop(&mut self) {
        unsafe { (self.impl_drop)(self) }
    }
}
impl<Output, const SIZE: usize> Future for FutureBox<Output, SIZE> {
    type Output = Output;

    fn poll(self: Pin<&mut Self>, context: &mut Context) -> Poll<Self::Output> {
        unsafe { (self.impl_poll)(self, context) }
    }
}
