use crate::Float32;

/// Returns `x` * 2<sup>`n`</sup>.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ldexpf(x: Float32, n: i32) -> Float32 {
    super::scalbnf(x, n)
}
