#![no_std]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod executor;
mod runtime;

pub use crate::executor::{Executor, FutureBox};
