#![feature(const_mut_refs)]

#![no_std]

#![warn(missing_docs, unused, clippy::all)]
#![doc = include_str!("../README.md")]

pub(crate) mod vectors;
pub(crate) mod traits;
pub(crate) mod coords;
pub(crate) mod angle;
pub(crate) mod types;
pub mod odo;

pub use llm;

pub mod prelude {
    //! Re-exports all the traits.
    pub use super::traits::*;
}

pub use vectors::*;
pub use coords::*;
pub use angle::*;
pub use types::*;
