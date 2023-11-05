use crate::Float32;

/// Returns the remainder of `x/y`.
#[inline]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn remainderf(x: Float32, y: Float32) -> Float32 {
    super::remquof(x, y).0
}
