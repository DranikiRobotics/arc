use crate::Float32;

use super::lgammaf_r;

/// Natural logarithm of gamma function
/// 
/// Returns the natural logarithm of the absolute value of the gamma function of x.
#[inline]
#[export_name = "__l2math_lgammaf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lgammaf(x: Float32) -> Float32 {
    lgammaf_r(x).0
}
