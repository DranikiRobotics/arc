use crate::Float32;

use super::log1pf;

/// Return the natural logarithm of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lnf(x: Float32) -> Float32 {
    log1pf(x - 1.)
}
