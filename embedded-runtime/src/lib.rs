#![no_std]
#![doc = include_str!("../README.md")]

pub mod spin;
pub mod error;
pub mod executor;
mod runtime;

pub use crate::{spin::spin_once, executor::Executor};
