use crate::Float64;

use super::log1p;

/// Return the natural logarithm of `x`.
#[inline]
#[export_name = "__llm_ln"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ln(x: Float64) -> Float64 {
    log1p(x - 1.)
}
