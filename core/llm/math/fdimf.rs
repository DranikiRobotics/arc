use crate::Float32;

/// Positive difference
///
/// Determines the positive difference between arguments, returning:
/// * x - y	if x > y, or
/// * +0	if x <= y, or
/// * NAN	if either argument is NAN.
///
/// A range error may occur.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn fdimf(x: Float32, y: Float32) -> Float32 {
    if x.is_nan() {
        x
    } else if y.is_nan() {
        y
    } else if x > y {
        x - y
    } else {
        0.0
    }
}
