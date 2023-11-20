/* origin: FreeBSD /usr/src/lib/msun/src/s_cbrtf.c */
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
 * Debugged and optimized by Bruce D. Evans.
*/
/**
 * cbrtf(x)
 * Return cube root of x
*/

use crate::{Float64, Float32};

const B1: u32 = 709958130; /* B1 = (127-127.0/3-0.03306235651)*2**23 */
const B2: u32 = 642849266; /* B2 = (127-127.0/3-24/3-0.03306235651)*2**23 */

/// Cube root
///
/// Computes the cube root of the argument.
#[export_name = "__llm_cbrtf"]
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub extern "C" fn cbrtf(x: Float32) -> Float32 {
    let x1p24 = Float32::from_bits(0x4b800000); // 0x1p24f === 2 ^ 24

    let mut r: Float64;
    let mut t: Float64;
    let mut ui: u32 = x.to_bits();
    let mut hx: u32 = ui & 0x7fffffff;

    if hx >= 0x7f800000 {
        /* cbrt(NaN,INF) is itself */
        return x + x;
    }

    /* rough cbrt to 5 bits */
    if hx < 0x00800000 {
        /* zero or subnormal? */
        if hx == 0 {
            return x; /* cbrt(+-0) is itself */
        }
        ui = (x * x1p24).to_bits();
        hx = ui & 0x7fffffff;
        hx = hx / 3 + B2;
    } else {
        hx = hx / 3 + B1;
    }
    ui &= 0x80000000;
    ui |= hx;

    /*
     * First step Newton iteration (solving t*t-x/t == 0) to 16 bits.  In
     * double precision so that its terms can be arranged for efficiency
     * without causing overflow or underflow.
     */
    t = Float32::from_bits(ui) as Float64;
    r = t * t * t;
    t = t * (x as Float64 + x as Float64 + r) / (x as Float64 + r + r);

    /*
     * Second step Newton iteration to 47 bits.  In double precision for
     * efficiency and accuracy.
     */
    r = t * t * t;
    t = t * (x as Float64 + x as Float64 + r) / (x as Float64 + r + r);

    /* rounding to 24 bits is perfect in round-to-nearest mode */
    t as Float32
}
