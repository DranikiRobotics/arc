use crate::Float32;

#[allow(clippy::tabs_in_doc_comments)]
/// Positive difference
///
/// Determines the positive difference between arguments, returning:
/// * x - y	if x > y, or
/// * +0	if x <= y, or
/// * NAN	if either argument is NAN.
///
/// A range error may occur.
#[export_name = "__l2math_fdimf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn fdimf(x: Float32, y: Float32) -> Float32 {
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
