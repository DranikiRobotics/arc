#![no_std]
#![feature(core_intrinsics)]
#![warn(missing_docs, unused, clippy::all)]
#![doc = include_str!("./README.md")]

extern crate alloc;

mod control;
mod math;

pub use control::*;
pub use math::*;
