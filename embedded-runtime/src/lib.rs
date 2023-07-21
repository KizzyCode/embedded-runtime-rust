#![no_std]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod executor;
mod runtime;
pub mod yield_;

pub use crate::{executor::Executor, yield_::yield_now};
