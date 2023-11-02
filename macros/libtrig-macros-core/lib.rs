//! A collection of core functionality for the macros.

#![warn(missing_docs, unused, clippy::all)]

use proc_macro2::TokenStream;

mod mass_impl;
mod ffi;

pub use mass_impl::mass_impl;
pub use ffi::ffi;
