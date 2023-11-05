use crate::Float64;

/// Returns the remainder of `x/y`.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn remainder(x: Float64, y: Float64) -> Float64 {
    super::remquo(x, y).0
}
