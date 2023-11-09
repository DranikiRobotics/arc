/* origin: FreeBSD /usr/src/lib/msun/src/k_tan.c */
/**
 * ====================================================
 * Copyright 2004 Sun Microsystems, Inc.  All Rights Reserved.
 *
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
*/

use crate::{Float64, Float32};

/* |tan(x)/x - t(x)| < 2**-25.5 (~[-2e-08, 2e-08]). */
const T: [Float64; 6] = [
    0.333_331_395_030_791_4,   /* 0x15554d3418c99f.0p-54 */
    0.133_392_002_712_976_74,   /* 0x1112fd38999f72.0p-55 */
    0.053_381_237_844_567_04,  /* 0x1b54c91d865afe.0p-57 */
    0.024_528_318_116_654_728,  /* 0x191df3908c33ce.0p-58 */
    0.002_974_357_433_599_673, /* 0x185dadfcecf44e.0p-61 */
    0.009_465_647_849_436_732, /* 0x1362b9bf971bcd.0p-59 */
];

#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub(crate) fn k_tanf(x: Float64, odd: bool) -> Float32 {
    let z = x * x;
    /*
     * Split up the polynomial into small independent terms to give
     * opportunities for parallel evaluation.  The chosen splitting is
     * micro-optimized for Athlons (XP, X64).  It costs 2 multiplications
     * relative to Horner's method on sequential machines.
     *
     * We add the small terms from lowest degree up for efficiency on
     * non-sequential machines (the lowest degree terms tend to be ready
     * earlier).  Apart from this, we don't care about order of
     * operations, and don't need to to care since we have precision to
     * spare.  However, the chosen splitting is good for accuracy too,
     * and would give results as accurate as Horner's method if the
     * small terms were added from highest degree down.
     */
    let mut r = T[4] + z * T[5];
    let t = T[2] + z * T[3];
    let w = z * z;
    let s = z * x;
    let u = T[0] + z * T[1];
    r = (x + s * u) + (s * w) * (t + w * r);
    (if odd { -1. / r } else { r }) as Float32
}
