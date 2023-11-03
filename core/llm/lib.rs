#![doc = include_str!("./README.md")]

#![no_std]
#![feature(core_intrinsics)]

#![allow(clippy::unreadable_literal)]
#![allow(clippy::many_single_char_names)]
#![allow(clippy::needless_return)]
#![allow(clippy::int_plus_one)]
#![allow(clippy::deprecated_cfg_attr)]
#![allow(clippy::mixed_case_hex_literals)]
#![allow(clippy::float_cmp)]
#![allow(clippy::eq_op)]
#![allow(clippy::assign_op_pattern)]

mod types;
mod math;

pub use self::math::*;
pub use types::*;

/// Approximate equality with 1 ULP of tolerance
#[doc(hidden)]
#[inline]
pub fn _eqf(a: Float32, b: Float32) -> Result<(), u32> {
    if a.is_nan() && b.is_nan() {
        Ok(())
    } else {
        let err = (a.to_bits() as i32).wrapping_sub(b.to_bits() as i32).abs();

        if err <= 1 {
            Ok(())
        } else {
            Err(err as u32)
        }
    }
}

#[doc(hidden)]
#[inline]
pub fn _eq(a: Float, b: Float) -> Result<(), u64> {
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
