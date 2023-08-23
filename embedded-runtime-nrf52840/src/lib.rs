#![no_std]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub mod runtime;

// Re-export everything
pub use embedded_runtime::*;
