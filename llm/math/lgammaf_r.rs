/* origin: FreeBSD /usr/src/lib/msun/src/e_lgammaf_r.c */
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
/**
 * Conversion to float by Ian Lance Taylor, Cygnus Support, ian@cygnus.com.
*/

use crate::{Float64, Float32};

use super::{floorf, k_cosf, k_sinf, logf};

const PI: Float32 = 3.141_592_7; /* 0x40490fdb */
const A0: Float32 = 7.721_566_4e-2; /* 0x3d9e233f */
const A1: Float32 = 3.224_670_3e-1; /* 0x3ea51a66 */
const A2: Float32 = 6.735_23e-2; /* 0x3d89f001 */
const A3: Float32 = 2.058_080_8e-2; /* 0x3ca89915 */
const A4: Float32 = 7.385_551e-3; /* 0x3bf2027e */
const A5: Float32 = 2.890_513_7e-3; /* 0x3b3d6ec6 */
const A6: Float32 = 1.192_707_7e-3; /* 0x3a9c54a1 */
const A7: Float32 = 5.100_698e-4; /* 0x3a05b634 */
const A8: Float32 = 2.208_627_8e-4; /* 0x39679767 */
const A9: Float32 = 1.080_115_7e-4; /* 0x38e28445 */
const A10: Float32 = 2.521_445_6e-5; /* 0x37d383a2 */
const A11: Float32 = 4.486_409_7e-5; /* 0x383c2c75 */
const TC: Float32 = 1.461_632_1; /* 0x3fbb16c3 */
const TF: Float32 = -1.214_862_84e-1; /* 0xbdf8cdcd */
/* TT = -(tail of TF) */
const TT: Float32 = 6.697_100_7e-9; /* 0x31e61c52 */
const T0: Float32 = 4.838_361e-1; /* 0x3ef7b95e */
const T1: Float32 = -1.475_877_2e-1; /* 0xbe17213c */
const T2: Float32 = 6.462_494e-2; /* 0x3d845a15 */
const T3: Float32 = -3.278_854e-2; /* 0xbd064d47 */
const T4: Float32 = 1.797_067_6e-2; /* 0x3c93373d */
const T5: Float32 = -1.031_422_4e-2; /* 0xbc28fcfe */
const T6: Float32 = 6.100_538_7e-3; /* 0x3bc7e707 */
const T7: Float32 = -3.684_520_3e-3; /* 0xbb7177fe */
const T8: Float32 = 2.259_647_7e-3; /* 0x3b141699 */
const T9: Float32 = -1.403_464_7e-3; /* 0xbab7f476 */
const T10: Float32 = 8.810_818_5e-4; /* 0x3a66f867 */
const T11: Float32 = -5.385_953e-4; /* 0xba0d3085 */
const T12: Float32 = 3.156_320_6e-4; /* 0x39a57b6b */
const T13: Float32 = -3.127_541_6e-4; /* 0xb9a3f927 */
const T14: Float32 = 3.355_291_8e-4; /* 0x39afe9f7 */
const U0: Float32 = -7.721_566_4e-2; /* 0xbd9e233f */
const U1: Float32 = 6.328_270_4e-1; /* 0x3f2200f4 */
const U2: Float32 = 1.454_922_6; /* 0x3fba3ae7 */
const U3: Float32 = 9.777_175e-1; /* 0x3f7a4bb2 */
const U4: Float32 = 2.289_637_3e-1; /* 0x3e6a7578 */
const U5: Float32 = 1.338_109_2e-2; /* 0x3c5b3c5e */
const V1: Float32 = 2.455_978; /* 0x401d2ebe */
const V2: Float32 = 2.128_489_7; /* 0x4008392d */
const V3: Float32 = 7.692_851_4e-1; /* 0x3f44efdf */
const V4: Float32 = 1.042_226_5e-1; /* 0x3dd572af */
const V5: Float32 = 3.217_092_5e-3; /* 0x3b52d5db */
const S0: Float32 = -7.721_566_4e-2; /* 0xbd9e233f */
const S1: Float32 = 2.149_824_2e-1; /* 0x3e5c245a */
const S2: Float32 = 3.257_787_8e-1; /* 0x3ea6cc7a */
const S3: Float32 = 1.463_504_7e-1; /* 0x3e15dce6 */
const S4: Float32 = 2.664_227e-2; /* 0x3cda40e4 */
const S5: Float32 = 1.840_284_6e-3; /* 0x3af135b4 */
const S6: Float32 = 3.194_753_3e-5; /* 0x3805ff67 */
const R1: Float32 = 1.392_005_3; /* 0x3fb22d3b */
const R2: Float32 = 7.219_356e-1; /* 0x3f38d0c5 */
const R3: Float32 = 1.719_338_6e-1; /* 0x3e300f6e */
const R4: Float32 = 1.864_592e-2; /* 0x3c98bf54 */
const R5: Float32 = 7.779_425e-4; /* 0x3a4beed6 */
const R6: Float32 = 7.326_684e-6; /* 0x36f5d7bd */
const W0: Float32 = 4.189_385_5e-1; /* 0x3ed67f1d */
const W1: Float32 = 8.333_333_6e-2; /* 0x3daaaaab */
const W2: Float32 = -2.777_777_8e-3; /* 0xbb360b61 */
const W3: Float32 = 7.936_506e-4; /* 0x3a500cfd */
const W4: Float32 = -5.951_875_4e-4; /* 0xba1c065c */
const W5: Float32 = 8.363_399e-4; /* 0x3a5b3dd2 */
const W6: Float32 = -1.630_929_3e-3; /* 0xbad5c4e8 */

/* sin(PI*x) assuming x > 2^-100, if sin(PI*x)==0 the sign is arbitrary */
fn sin_pi(mut x: Float32) -> Float32 {
    let mut y: Float64;
    let mut n: isize;

    /* spurious inexact if odd int */
    x = 2.0 * (x * 0.5 - floorf(x * 0.5)); /* x mod 2.0 */

    n = (x * 4.0) as isize;
    n = div!(n + 1, 2);
    y = (x as Float64) - (n as Float64) * 0.5;
    y *= 3.141_592_653_589_793;
    match n {
        1 => k_cosf(y),
        2 => k_sinf(-y),
        3 => -k_cosf(y),
        0 | _ => k_sinf(y),
    }
}

/// Natural logarithm of gamma function
/// 
/// Returns the natural logarithm of the absolute value of the gamma function of x,
/// and the sign of the gamma function of x
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lgammaf_r(mut x: Float32) -> (Float32, i32) {
    let u = x.to_bits();
    let mut t: Float32;
    let y: Float32;
    let mut z: Float32;
    let nadj: Float32;
    let p: Float32;
    let p1: Float32;
    let p2: Float32;
    let p3: Float32;
    let q: Float32;
    let mut r: Float32;
    let w: Float32;
    
    let i: i32;
    
    let mut signgam: i32;

    /* purge off +-inf, NaN, +-0, tiny and negative arguments */
    signgam = 1;
    let sign: bool = (u >> 31) != 0;
    let ix: u32 = u & 0x7fffffff;
    if ix >= 0x7f800000 {
        return (x * x, signgam);
    }
    if ix < 0x35000000 {
        /* |x| < 2**-21, return -log(|x|) */
        if sign {
            signgam = -1;
            x = -x;
        }
        return (-logf(x), signgam);
    }
    if sign {
        x = -x;
        t = sin_pi(x);
        if t == 0.0 {
            /* -integer */
            return (1.0 / (x - x), signgam);
        }
        if t > 0.0 {
            signgam = -1;
        } else {
            t = -t;
        }
        nadj = logf(PI / (t * x));
    } else {
        nadj = 0.0;
    }

    /* purge off 1 and 2 */
    if ix == 0x3f800000 || ix == 0x40000000 {
        r = 0.0;
    }
    /* for x < 2.0 */
    else if ix < 0x40000000 {
        if ix <= 0x3f666666 {
            /* lgamma(x) = lgamma(x+1)-log(x) */
            r = -logf(x);
            if ix >= 0x3f3b4a20 {
                y = 1.0 - x;
                i = 0;
            } else if ix >= 0x3e6d3308 {
                y = x - (TC - 1.0);
                i = 1;
            } else {
                y = x;
                i = 2;
            }
        } else {
            r = 0.0;
            if ix >= 0x3fdda618 {
                /* [1.7316,2] */
                y = 2.0 - x;
                i = 0;
            } else if ix >= 0x3F9da620 {
                /* [1.23,1.73] */
                y = x - TC;
                i = 1;
            } else {
                y = x - 1.0;
                i = 2;
            }
        }
        match i {
            0 => {
                z = y * y;
                p1 = A0 + z * (A2 + z * (A4 + z * (A6 + z * (A8 + z * A10))));
                p2 = z * (A1 + z * (A3 + z * (A5 + z * (A7 + z * (A9 + z * A11)))));
                p = y * p1 + p2;
                r += p - 0.5 * y;
            }
            1 => {
                z = y * y;
                w = z * y;
                p1 = T0 + w * (T3 + w * (T6 + w * (T9 + w * T12))); /* parallel comp */
                p2 = T1 + w * (T4 + w * (T7 + w * (T10 + w * T13)));
                p3 = T2 + w * (T5 + w * (T8 + w * (T11 + w * T14)));
                p = z * p1 - (TT - w * (p2 + y * p3));
                r += TF + p;
            }
            2 => {
                p1 = y * (U0 + y * (U1 + y * (U2 + y * (U3 + y * (U4 + y * U5)))));
                p2 = 1.0 + y * (V1 + y * (V2 + y * (V3 + y * (V4 + y * V5))));
                r += -0.5 * y + p1 / p2;
            }
            #[cfg(debug_assertions)]
            _ => unreachable!(),
            #[cfg(not(debug_assertions))]
            _ => {}
        }
    } else if ix < 0x41000000 {
        /* x < 8.0 */
        i = x as i32;
        y = x - (i as Float32);
        p = y * (S0 + y * (S1 + y * (S2 + y * (S3 + y * (S4 + y * (S5 + y * S6))))));
        q = 1.0 + y * (R1 + y * (R2 + y * (R3 + y * (R4 + y * (R5 + y * R6)))));
        r = 0.5 * y + p / q;
        z = 1.0; /* lgamma(1+s) = log(s) + lgamma(s) */
        // TODO: In C, this was implemented using switch jumps with fallthrough.
        // Does this implementation have performance problems?
        if i >= 7 {
            z *= y + 6.0;
        }
        if i >= 6 {
            z *= y + 5.0;
        }
        if i >= 5 {
            z *= y + 4.0;
        }
        if i >= 4 {
            z *= y + 3.0;
        }
        if i >= 3 {
            z *= y + 2.0;
            r += logf(z);
        }
    } else if ix < 0x5c800000 {
        /* 8.0 <= x < 2**58 */
        t = logf(x);
        z = 1.0 / x;
        y = z * z;
        w = W0 + z * (W1 + y * (W2 + y * (W3 + y * (W4 + y * (W5 + y * W6)))));
        r = (x - 0.5) * (t - 1.0) + w;
    } else {
        /* 2**58 <= x <= inf */
        r = x * (logf(x) - 1.0);
    }
    if sign {
        r = nadj - r;
    }
    return (r, signgam);
}
