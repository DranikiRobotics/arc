use crate::Float32;

use super::ln1pf;

/// Return the natural logarithm of `x`.
#[inline]
#[export_name = "__l2math_lnf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lnf(x: Float32) -> Float32 {
    ln1pf(x - 1.)
}
