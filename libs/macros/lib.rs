#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all)]

use proc_macro::TokenStream;

/// A macro for implementing a trait for a list of types.
#[proc_macro_attribute]
pub fn mass_impl(cfg: TokenStream, input: TokenStream) -> TokenStream {
    mc::mass_impl(cfg, input).into()
}

/// A macro for generating FFI functions.
#[proc_macro_attribute]
pub fn ffi(cfg: TokenStream, input: TokenStream) -> TokenStream {
    mc::ffi(cfg, input).into()
}

/// A macro for generating function modifications.
#[proc_macro_attribute]
pub fn func_mod(cfg: TokenStream, input: TokenStream) -> TokenStream {
    mc::func_mod(cfg, input).into()
}
