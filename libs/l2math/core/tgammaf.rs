use crate::{Float64, Float32};

use super::tgamma;

/// Returns the gamma function of `x`.
#[inline]
#[export_name = "__l2math_tgammaf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn tgammaf(x: Float32) -> Float32 {
    tgamma(x as Float64) as Float32
}
