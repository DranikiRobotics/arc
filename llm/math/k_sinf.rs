/* origin: FreeBSD /usr/src/lib/msun/src/k_sinf.c */
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
 * Optimized by Bruce D. Evans.
*/

use crate::{Float64, Float32};

/* |sin(x)/x - s(x)| < 2**-37.5 (~[-4.89e-12, 4.824e-12]). */
consts!{
const S1: Float64 = -0.166666666416265235595; /* -0x15555554cbac77.0p-55 */
const S2: Float64 = 0.0083333293858894631756; /*  0x111110896efbb2.0p-59 */
const S3: Float64 = -0.000198393348360966317347; /* -0x1a00f9e2cae774.0p-65 */
const S4: Float64 = 0.0000027183114939898219064; /*  0x16cd878c3b46a7.0p-71 */
}

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_sinf(x: Float64) -> Float32 {
    let z = x * x;
    let w = z * z;
    let r = S3 + z * S4;
    let s = z * x;
    ((x + s * (S1 + z * S2)) + s * w * r) as Float32
}
