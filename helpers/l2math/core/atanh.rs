use crate::{Float64, Float32, Radian64};

use super::log1p;

/* atanh(x) = log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2 ~= x + x^3/3 + o(x^5) */
/// Inverse hyperbolic tangent
///
/// Calculates the inverse hyperbolic tangent of `x`.
/// Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.
#[export_name = "__l2math_atanh"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn atanh(x: Float64) -> Radian64 {
    let u = x.to_bits();
    let e = ((u >> 52) as usize) & 0x7ff;
    let sign = (u >> 63) != 0;

    /* |x| */
    let mut y = Float64::from_bits(u & 0x7fff_ffff_ffff_ffff);

    if e < 0x3ff - 1 {
        if e < 0x3ff - 32 {
            /* handle underflow */
            if e == 0 {
                force_eval!(y as Float32);
            }
        } else {
            /* |x| < 0.5, up to 1.7ulp error */
            y = 0.5 * log1p(2.0 * y + 2.0 * y * y / (1.0 - y));
        }
    } else {
        /* avoid overflow */
        y = 0.5 * log1p(2.0 * (y / (1.0 - y)));
    }

    if sign {
        -y
    } else {
        y
    }
}
