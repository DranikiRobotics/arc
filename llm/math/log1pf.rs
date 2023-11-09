/* origin: FreeBSD /usr/src/lib/msun/src/s_log1pf.c */
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

use crate::Float32;

const LN2_HI: Float32 = 6.931_381e-1; /* 0x3f317180 */
const LN2_LO: Float32 = 9.058_001e-6; /* 0x3717f7d1 */
/* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95e-11, 4.97e-11]). */
const LG1: Float32 = 0.666_666_6; /* 0xaaaaaa.0p-24 */
const LG2: Float32 = 0.400_009_72; /* 0xccce13.0p-25 */
const LG3: Float32 = 0.284_987_87; /* 0x91e9ee.0p-25 */
const LG4: Float32 = 0.242_790_79; /* 0xf89e26.0p-26 */

/// Return the natural logarithm of `1+x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log1pf(x: Float32) -> Float32 {
    let mut ui: u32 = x.to_bits();
    
    let mut f: Float32 = 0.;
    let mut c: Float32 = 0.;
    
    
    
    
    
    
    
    
    let mut iu: u32;
    let mut k: i32;

    let ix: u32 = ui;
    k = 1;
    if ix < 0x3ed413d0 || (ix >> 31) > 0 {
        /* 1+x < sqrt(2)+  */
        if ix >= 0xbf800000 {
            /* x <= -1.0 */
            if x == -1. {
                return x / 0.0; /* log1p(-1)=+inf */
            }
            return (x - x) / 0.0; /* log1p(x<-1)=NaN */
        }
        if ix << 1 < 0x33800000 << 1 {
            /* |x| < 2**-24 */
            /* underflow if subnormal */
            if (ix & 0x7f800000) == 0 {
                force_eval!(x * x);
            }
            return x;
        }
        if ix <= 0xbe95f619 {
            /* sqrt(2)/2- <= 1+x < sqrt(2)+ */
            k = 0;
            c = 0.;
            f = x;
        }
    } else if ix >= 0x7f800000 {
        return x;
    }
    if k > 0 {
        ui = (1. + x).to_bits();
        iu = ui;
        iu += 0x3f800000 - 0x3f3504f3;
        k = (iu >> 23) as i32 - 0x7f;
        /* correction term ~ log(1+x)-log(u), avoid underflow in c/u */
        if k < 25 {
            c = if k >= 2 {
                1. - (Float32::from_bits(ui) - x)
            } else {
                x - (Float32::from_bits(ui) - 1.)
            };
            c /= Float32::from_bits(ui);
        } else {
            c = 0.;
        }
        /* reduce u into [sqrt(2)/2, sqrt(2)] */
        iu = (iu & 0x007fffff) + 0x3f3504f3;
        ui = iu;
        f = Float32::from_bits(ui) - 1.;
    }
    let s: Float32 = f / (2.0 + f);
    let z: Float32 = s * s;
    let w: Float32 = z * z;
    let t1: Float32 = w * (LG2 + w * LG4);
    let t2: Float32 = z * (LG1 + w * LG3);
    let r: Float32 = t2 + t1;
    let hfsq: Float32 = 0.5 * f * f;
    let dk: Float32 = k as Float32;
    s * (hfsq + r) + (dk * LN2_LO + c) - hfsq + f + dk * LN2_HI
}
