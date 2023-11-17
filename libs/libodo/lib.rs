#![no_std]
#![feature(core_intrinsics)]
#![warn(missing_docs, unused, clippy::all)]
#![doc = include_str!("./README.md")]

extern crate alloc;

mod dualnum;
mod rotation2d;

pub use dualnum::DualNum;
pub use rotation2d::Rotation2D;
