//! A collection of core functionality for the macros.

#![warn(missing_docs, unused, clippy::all)]

use proc_macro2::TokenStream;

mod ffi;
mod mass_impl;

pub use ffi::ffi;
pub use mass_impl::mass_impl;
