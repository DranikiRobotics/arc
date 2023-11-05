use crate::Float64;

/// Returns `x` * 2<sup>`n`</sup>.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn ldexp(x: Float64, n: i32) -> Float64 {
    super::scalbn(x, n)
}
