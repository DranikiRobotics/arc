use crate::Float64;

/// Returns the remainder of `x/y`.
#[inline]
#[export_name = "__llm_remainder"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn remainder(x: Float64, y: Float64) -> Float64 {
    super::remquo(x, y).0
}
