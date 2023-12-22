use crate::{Float64, Int};

/// Breaks the given number into an integral and a fractional part.
pub fn modf(x: Float64) -> (Float64, Float64) {
    let rv2: Float64;
    let mut u = x.to_bits();
    let e = ((u >> 52 & 0x7ff) as Int) - 0x3ff;

    /* no fractional part */
    if e >= 52 {
        rv2 = x;
        if e == 0x400 && (u << 12) != 0 {
            /* nan */
            return (x, rv2);
        }
        u &= 1 << 63;
        return (Float64::from_bits(u), rv2);
    }

    /* no integral part*/
    if e < 0 {
        u &= 1 << 63;
        rv2 = Float64::from_bits(u);
        return (x, rv2);
    }

    let mask: u64 = ((!0) >> 12) >> e;
    if (u & mask) == 0 {
        rv2 = x;
        u &= 1 << 63;
        return (Float64::from_bits(u), rv2);
    }
    u &= !mask;
    rv2 = Float64::from_bits(u);
    return (x - rv2, rv2);
}

/// FFI bindings for modf
#[inline]
#[doc(hidden)]
#[export_name = "__l2math_modf"]
pub extern "C" fn __l2math_modf(x: Float64) -> super::Tuple_Float64_Float64 {
    super::Tuple_Float64_Float64::from(modf(x))
}
