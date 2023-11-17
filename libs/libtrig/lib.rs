#![no_std]
#![cfg_attr(feature = "unstable", feature(const_fn_floating_point_arithmetic))]
#![cfg_attr(feature = "unstable", feature(const_mut_refs))]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![doc = include_str!("./README.md")]

#[path = "u2/mod.rs"]
mod _u2;
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

pub use _u2::u2;
pub use angle::Angle2D;
pub use coords::Coord2D;
pub use types::*;
pub use vectors::{Vec2D, Vec3D};
