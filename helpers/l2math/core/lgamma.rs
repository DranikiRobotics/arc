use crate::Float64;

use super::lgamma_r;

/// Natural logarithm of gamma function
/// 
/// Returns the natural logarithm of the absolute value of the gamma function of x.
#[inline]
#[export_name = "__l2math_lgamma"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn lgamma(x: Float64) -> Float64 {
    lgamma_r(x).0
}
