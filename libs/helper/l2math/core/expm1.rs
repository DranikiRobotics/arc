/* origin: FreeBSD /usr/src/lib/msun/src/s_expm1.c */
/**
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunPro, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
*/

use crate::{Float64, Int};

consts!{
const O_THRESHOLD: Float64 = 7.09782712893383973096e+02; /* 0x40862E42, 0xFEFA39EF */
const LN2_HI: Float64 = 6.93147180369123816490e-01; /* 0x3fe62e42, 0xfee00000 */
const LN2_LO: Float64 = 1.90821492927058770002e-10; /* 0x3dea39ef, 0x35793c76 */
const INVLN2: Float64 = 1.44269504088896338700e+00; /* 0x3ff71547, 0x652b82fe */
/* Scaled Q's: Qn_here = 2**n * Qn_above, for R(2*z) where z = hxs = x*x/2: */
const Q1: Float64 = -3.33333333333331316428e-02; /* BFA11111 111110F4 */
const Q2: Float64 = 1.58730158725481460165e-03; /* 3F5A01A0 19FE5585 */
const Q3: Float64 = -7.93650757867487942473e-05; /* BF14CE19 9EAADBB7 */
const Q4: Float64 = 4.00821782732936239552e-06; /* 3ED0CFCA 86E65239 */
const Q5: Float64 = -2.01099218183624371326e-07; /* BE8AFDB7 6E09C32D */
}

/// Exponential, base *e*, of x-1
///
/// Calculates the exponential of `x` and subtract 1, that is, *e* raised
/// to the power `x` minus 1 (where *e* is the base of the natural
/// system of logarithms, approximately 2.71828).
/// The result is accurate even for small values of `x`,
/// where using `exp(x)-1` would lose many significant digits.
#[export_name = "__l2math_expm1"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn expm1(mut x: Float64) -> Float64 {
    let hi: Float64;
    let lo: Float64;
    let k: Int;
    let c: Float64;
    let mut t: Float64;
    let mut y: Float64;

    let mut ui = x.to_bits();
    let hx = ((ui >> 32) & 0x7fffffff) as u32;
    let sign = (ui >> 63) as Int;

    /* filter out huge and non-finite argument */
    if hx >= 0x4043687A {
        /* if |x|>=56*ln2 */
        if x.is_nan() {
            return x;
        }
        if sign != 0 {
            return -1.0;
        }
        if x > O_THRESHOLD {
            x *= Float64::from_bits(0x7fe0000000000000);
            return x;
        }
    }

    /* argument reduction */
    if hx > 0x3fd62e42 {
        /* if  |x| > 0.5 ln2 */
        if hx < 0x3FF0A2B2 {
            /* and |x| < 1.5 ln2 */
            if sign == 0 {
                hi = x - LN2_HI;
                lo = LN2_LO;
                k = 1;
            } else {
                hi = x + LN2_HI;
                lo = -LN2_LO;
                k = -1;
            }
        } else {
            k = (INVLN2 * x + if sign != 0 { -0.5 } else { 0.5 }) as Int;
            t = k as Float64;
            hi = x - t * LN2_HI; /* t*ln2_hi is exact here */
            lo = t * LN2_LO;
        }
        x = hi - lo;
        c = (hi - x) - lo;
    } else if hx < 0x3c900000 {
        /* |x| < 2**-54, return x */
        if hx < 0x00100000 {
            force_eval!(x);
        }
        return x;
    } else {
        c = 0.0;
        k = 0;
    }

    /* x is now in primary range */
    let hfx = 0.5 * x;
    let hxs = x * hfx;
    let r1 = 1.0 + hxs * (Q1 + hxs * (Q2 + hxs * (Q3 + hxs * (Q4 + hxs * Q5))));
    t = 3.0 - r1 * hfx;
    let mut e = hxs * ((r1 - t) / (6.0 - x * t));
    if k == 0 {
        /* c is 0 */
        return x - (x * e - hxs);
    }
    e = x * (e - c) - c;
    e -= hxs;
    /* exp(x) ~ 2^k (x_reduced - e + 1) */
    if k == -1 {
        return 0.5 * (x - e) - 0.5;
    }
    if k == 1 {
        if x < -0.25 {
            return -2.0 * (e - (x + 0.5));
        }
        return 1.0 + 2.0 * (x - e);
    }
    ui = ((0x3ff + k) as u64) << 52; /* 2^k */
    let twopk = Float64::from_bits(ui);
    if !(0..=56).contains(&k) {
        /* suffice to return exp(x)-1 */
        y = x - e + 1.0;
        if k == 1024 {
            y = y * 2.0 * Float64::from_bits(0x7fe0000000000000);
        } else {
            y = y * twopk;
        }
        return y - 1.0;
    }
    ui = ((0x3ff - k) as u64) << 52; /* 2^-k */
    let uf = Float64::from_bits(ui);
    if k < 20 {
        y = (x - e + (1.0 - uf)) * twopk;
    } else {
        y = (x - (e + uf) + 1.0) * twopk;
    }
    y
}

#[cfg(test)]
mod tests {
    #[test]
    fn sanity_check() {
        assert_eq!(super::expm1(1.1), 2.0041660239464334);
    }
}
