use crate::{Float64, Float32};

use super::tgamma;

/// Returns the gamma function of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn tgammaf(x: Float32) -> Float32 {
    tgamma(x as Float64) as Float32
}
