#![feature(const_mut_refs)]
#![no_std]
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

pub use angle::*;
pub use coords::*;
pub use types::*;
pub use vectors::*;
