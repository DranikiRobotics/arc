/* origin: FreeBSD /usr/src/lib/msun/src/e_log10.c */
/**
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
*/
/**
 * Return the base 10 logarithm of x.  See log.c for most comments.
 *
 * Reduce x to 2^k (1+f) and calculate r = log(1+f) - f + f*f/2
 * as in log.c, then combine and scale in extra precision:
 *    log10(x) = (f - f*f/2 + r)/log(10) + k*log10(2)
*/

use crate::Float64;

const IVLN10HI: Float64 = 4.34294481878168880939e-01; /* 0x3fdbcb7b, 0x15200000 */
const IVLN10LO: Float64 = 2.50829467116452752298e-11; /* 0x3dbb9438, 0xca9aadd5 */
const LOG10_2HI: Float64 = 3.01029995663611771306e-01; /* 0x3FD34413, 0x509F6000 */
const LOG10_2LO: Float64 = 3.69423907715893078616e-13; /* 0x3D59FEF3, 0x11F12B36 */
const LG1: Float64 = 6.666666666666735130e-01; /* 3FE55555 55555593 */
const LG2: Float64 = 3.999999999940941908e-01; /* 3FD99999 9997FA04 */
const LG3: Float64 = 2.857142874366239149e-01; /* 3FD24924 94229359 */
const LG4: Float64 = 2.222219843214978396e-01; /* 3FCC71C5 1D8E78AF */
const LG5: Float64 = 1.818357216161805012e-01; /* 3FC74664 96CB03DE */
const LG6: Float64 = 1.531383769920937332e-01; /* 3FC39A09 D078C69F */
const LG7: Float64 = 1.479819860511658591e-01; /* 3FC2F112 DF3E5244 */

/// Returns the base 10 logarithm of `x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log10(mut x: Float64) -> Float64 {
    let x1p54 = Float64::from_bits(0x4350000000000000); // 0x1p54 === 2 ^ 54

    let mut ui: u64 = x.to_bits();
    let hfsq: Float64;
    let f: Float64;
    let s: Float64;
    let z: Float64;
    let r: Float64;
    let mut w: Float64;
    let t1: Float64;
    let t2: Float64;
    let dk: Float64;
    let y: Float64;
    let mut hi: Float64;
    let lo: Float64;
    let mut val_hi: Float64;
    let mut val_lo: Float64;
    let mut hx: u32;
    let mut k: i32;

    hx = (ui >> 32) as u32;
    k = 0;
    if hx < 0x00100000 || (hx >> 31) > 0 {
        if ui << 1 == 0 {
            return -1. / (x * x); /* log(+-0)=-inf */
        }
        if (hx >> 31) > 0 {
            return (x - x) / 0.0; /* log(-#) = NaN */
        }
        /* subnormal number, scale x up */
        k -= 54;
        x *= x1p54;
        ui = x.to_bits();
        hx = (ui >> 32) as u32;
    } else if hx >= 0x7ff00000 {
        return x;
    } else if hx == 0x3ff00000 && ui << 32 == 0 {
        return 0.;
    }

    /* reduce x into [sqrt(2)/2, sqrt(2)] */
    hx += 0x3ff00000 - 0x3fe6a09e;
    k += (hx >> 20) as i32 - 0x3ff;
    hx = (hx & 0x000fffff) + 0x3fe6a09e;
    ui = (hx as u64) << 32 | (ui & 0xffffffff);
    x = Float64::from_bits(ui);

    f = x - 1.0;
    hfsq = 0.5 * f * f;
    s = f / (2.0 + f);
    z = s * s;
    w = z * z;
    t1 = w * (LG2 + w * (LG4 + w * LG6));
    t2 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    r = t2 + t1;

    /* See log2.c for details. */
    /* hi+lo = f - hfsq + s*(hfsq+R) ~ log(1+f) */
    hi = f - hfsq;
    ui = hi.to_bits();
    ui &= (-1i64 as u64) << 32;
    hi = Float64::from_bits(ui);
    lo = f - hi - hfsq + s * (hfsq + r);

    /* val_hi+val_lo ~ log10(1+f) + k*log10(2) */
    val_hi = hi * IVLN10HI;
    dk = k as Float64;
    y = dk * LOG10_2HI;
    val_lo = dk * LOG10_2LO + (lo + hi) * IVLN10LO + lo * IVLN10HI;

    /*
     * Extra precision in for adding y is not strictly needed
     * since there is no very large cancellation near x = sqrt(2) or
     * x = 1/sqrt(2), but we do it anyway since it costs little on CPUs
     * with some parallelism and it reduces the error for many args.
     */
    w = y + val_hi;
    val_lo += (y - w) + val_hi;
    val_hi = w;

    val_lo + val_hi
}
