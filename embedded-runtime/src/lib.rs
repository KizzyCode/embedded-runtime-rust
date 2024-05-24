#![no_std]
#![doc = include_str!("../README.md")]

pub mod context;
pub mod error;
pub mod executor;
mod runtime;

pub use crate::{
    context::{sleep_once, spin_once},
    executor::Executor,
};
