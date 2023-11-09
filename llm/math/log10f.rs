/* origin: FreeBSD /usr/src/lib/msun/src/e_log10f.c */
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
 * See comments in log10.c.
*/

use crate::Float32;

const IVLN10HI: Float32 = 4.343_261_7e-1; /* 0x3ede6000 */
const IVLN10LO: Float32 = -3.168_997e-5; /* 0xb804ead9 */
const LOG10_2HI: Float32 = 3.010_292e-1; /* 0x3e9a2080 */
const LOG10_2LO: Float32 = 7.903_415e-7; /* 0x355427db */
/* |(log(1+s)-log(1-s))/s - Lg(s)| < 2**-34.24 (~[-4.95e-11, 4.97e-11]). */
const LG1: Float32 = 0.666_666_6; /* 0xaaaaaa.0p-24 */
const LG2: Float32 = 0.400_009_72; /* 0xccce13.0p-25 */
const LG3: Float32 = 0.284_987_87; /* 0x91e9ee.0p-25 */
const LG4: Float32 = 0.242_790_79; /* 0xf89e26.0p-26 */

/// Returns the base 10 logarithm of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log10f(mut x: Float32) -> Float32 {
    let x1p25f = Float32::from_bits(0x4c000000); // 0x1p25f === 2 ^ 25

    let mut ui: u32 = x.to_bits();
    
    
    
    
    
    
    
    
    
    let mut hi: Float32;
    
    let mut ix: u32;
    let mut k: i32;

    ix = ui;
    k = 0;
    if ix < 0x00800000 || (ix >> 31) > 0 {
        /* x < 2**-126  */
        if ix << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if (ix >> 31) > 0 {
            return (x - x) / 0.0; /* log(-#) = NaN */
        }
        /* subnormal number, scale up x */
        k -= 25;
        x *= x1p25f;
        ui = x.to_bits();
        ix = ui;
    } else if ix >= 0x7f800000 {
        return x;
    } else if ix == 0x3f800000 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    ix += 0x3f800000 - 0x3f3504f3;
    k += (ix >> 23) as i32 - 0x7f;
    ix = (ix & 0x007fffff) + 0x3f3504f3;
    ui = ix;
    x = Float32::from_bits(ui);

    let f: Float32 = x - 1.0;
    let s: Float32 = f / (2.0 + f);
    let z: Float32 = s * s;
    let w: Float32 = z * z;
    let t1: Float32 = w * (LG2 + w * LG4);
    let t2: Float32 = z * (LG1 + w * LG3);
    let r: Float32 = t2 + t1;
    let hfsq: Float32 = 0.5 * f * f;

    hi = f - hfsq;
    ui = hi.to_bits();
    ui &= 0xfffff000;
    hi = Float32::from_bits(ui);
    let lo: Float32 = f - hi - hfsq + s * (hfsq + r);
    let dk: Float32 = k as Float32;
    dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI + hi * IVLN10HI + dk * LOG10_2HI
}
