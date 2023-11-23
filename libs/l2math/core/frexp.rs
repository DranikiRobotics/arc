use crate::{Float64, Int};

/// Breaks the number into a normalized fraction and a base-2 exponent
/// 
/// Satisfying:
/// 
/// * `x = f * 2^exp`
/// * `0.5 <= abs(f) < 1.0`
pub fn frexp(x: Float64) -> (Float64, Int) {
    let mut y = x.to_bits();
    let ee = ((y >> 52) & 0x7ff) as Int;

    if ee == 0 {
        if x != 0.0 {
            let x1p64 = Float64::from_bits(0x43f0000000000000);
            let (x, e) = frexp(x * x1p64);
            return (x, e - 64);
        }
        return (x, 0);
    } else if ee == 0x7ff {
        return (x, 0);
    }

    let e = ee - 0x3fe;
    y &= 0x800fffffffffffff;
    y |= 0x3fe0000000000000;
    return (Float64::from_bits(y), e);
}

/// FFI bindings for frexp
#[inline]
#[doc(hidden)]
#[export_name = "__l2math_frexp"]
pub extern "C" fn __l2math_frexp(x: Float64) -> super::Tuple_Float64_Int {
    super::Tuple_Float64_Int::from(frexp(x))
}
