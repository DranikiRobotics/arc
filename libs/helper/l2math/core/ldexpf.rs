use crate::{Float32, Int};

/// Returns `x` * 2<sup>`n`</sup>.
#[inline]
#[export_name = "__l2math_ldexpf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ldexpf(x: Float32, n: Int) -> Float32 {
    super::scalbnf(x, n)
}
