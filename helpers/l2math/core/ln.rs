use crate::Float64;

use super::ln1p;

/// Return the natural logarithm of `x`.
#[inline]
#[export_name = "__l2math_ln"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln(x: Float64) -> Float64 {
    ln1p(x - 1.)
}
