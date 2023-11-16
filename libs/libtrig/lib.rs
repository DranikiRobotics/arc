#![feature(const_mut_refs)]
#![no_std]
#![feature(const_fn_floating_point_arithmetic)]
#![warn(missing_docs, unused, clippy::all)]
#![doc = include_str!("./README.md")]

pub(crate) mod angle;
pub(crate) mod coords;
pub(crate) mod traits;
pub(crate) mod types;
pub(crate) mod vectors;

pub use llm;

pub mod prelude {
    //! Re-exports all the traits.
    pub use super::traits::*;
}

pub use angle::Angle2D;
pub use coords::Coord2D;
pub use types::*;
pub use vectors::{Vec3D, Vec2D};
