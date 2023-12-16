#![no_std]
#![cfg_attr(feature = "unstable", feature(const_fn_floating_point_arithmetic))]
#![cfg_attr(feature = "unstable", feature(const_mut_refs))]
#![cfg_attr(feature = "unstable", feature(const_mut_refs))]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![doc = include_str!("./README.md")]

pub(crate) mod angle;
pub(crate) mod coords;
pub(crate) mod morenums;
pub(crate) mod traits;
pub(crate) mod types;
pub(crate) mod vectors;

pub mod prelude {
    //! Re-exports all the traits.
    pub use super::traits::*;
}

pub use angle::Angle2D;
pub use coords::Coord2D;
pub use morenums::{u2, u3};
pub use types::*;
pub use vectors::{Vec2D, Vec3D};

/// Re-exports all the traits.
#[macro_export]
macro_rules! prelude {
    () => (
        #[allow(unused_imports)]
        use $crate::prelude::*;
    );
}
