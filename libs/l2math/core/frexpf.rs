use crate::{Float32, Int};

/// Breaks the number into a normalized fraction and a base-2 exponent
/// 
/// Satisfying:
/// 
/// * `x = f * 2^exp`
/// * `0.5 <= abs(f) < 1.0`
pub fn frexpf(x: Float32) -> (Float32, Int) {
    let mut y = x.to_bits();
    let ee: Int = ((y >> 23) & 0xff) as Int;

    if ee == 0 {
        if x != 0.0 {
            let x1p64 = Float32::from_bits(0x5f800000);
            let (x, e) = frexpf(x * x1p64);
            return (x, e - 64);
        } else {
            return (x, 0);
        }
    } else if ee == 0xff {
        return (x, 0);
    }

    let e = ee - 0x7e;
    y &= 0x807fffff;
    y |= 0x3f000000;
    (Float32::from_bits(y), e)
}

/// FFI bindings for frexpf
#[inline]
#[doc(hidden)]
#[export_name = "__l2math_frexpf"]
pub extern "C" fn __l2math_frexpf(x: Float32) -> super::Tuple_Float32_Int {
    super::Tuple_Float32_Int::from(frexpf(x))
}
