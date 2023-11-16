#![no_std]
#![feature(core_intrinsics)]
#![warn(missing_docs, unused, clippy::all)]
#![doc = include_str!("./README.md")]

extern crate alloc;

mod rotation2d;
mod dualnum;

pub use rotation2d::Rotation2D;
pub use dualnum::DualNum;
