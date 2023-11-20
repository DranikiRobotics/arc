use crate::{Float64, Int};

/// Returns `x` * 2<sup>`n`</sup>.
#[inline]
#[export_name = "__llm_ldexp"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn ldexp(x: Float64, n: Int) -> Float64 {
    super::scalbn(x, n)
}
