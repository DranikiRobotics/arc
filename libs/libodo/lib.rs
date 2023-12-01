#![no_std]
#![feature(core_intrinsics)]
#![warn(missing_docs, unused, clippy::all)]
#![doc = include_str!("./README.md")]

extern crate alloc;

mod dualnum;
// mod rotation2d;
mod vec2ddual;

pub use dualnum::DualNum;
// pub use rotation2d::Rotation2D;
pub use vec2ddual::Vec2DDual;
