#![doc = include_str!("./README.md")]
#![no_std]
#![cfg_attr(feature = "unstable", feature(core_intrinsics))]
#![warn(missing_docs, unused, clippy::all)]
#![allow(unused_unsafe)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_return)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::deprecated_cfg_attr)]
#![allow(clippy::mixed_case_hex_literals)]
#![allow(clippy::float_cmp)]
#![allow(clippy::eq_op)]
#![allow(clippy::assign_op_pattern)]

mod core;
mod types;

#[path = "macros.rs"]
mod __macros;

pub use self::core::*;
pub use self::types::*;

/// Approximate equality with 1 ULP of tolerance
#[inline]
#[doc(hidden)]
pub(crate) fn _eqf(a: Float32, b: Float32) -> Result<(), u32> {
    if a.is_nan() && b.is_nan() {
        Ok(())
    } else {
        let err = (a.to_bits() as Int).wrapping_sub(b.to_bits() as Int).abs();

        if err <= 1 {
            Ok(())
        } else {
            Err(err as u32)
        }
    }
}

#[inline]
#[doc(hidden)]
pub(crate) fn _eq(a: Float64, b: Float64) -> Result<(), u64> {
    if a.is_nan() && b.is_nan() {
        Ok(())
    } else {
        let err = (a.to_bits() as i64).wrapping_sub(b.to_bits() as i64).abs();

        if err <= 1 {
            Ok(())
        } else {
            Err(err as u64)
        }
    }
}
