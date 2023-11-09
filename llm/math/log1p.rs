/* origin: FreeBSD /usr/src/lib/msun/src/s_log1p.c */
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
/*
 * double log1p(double x)
 * Return the natural logarithm of 1+x.
 *
 * Method :
 *   1. Argument Reduction: find k and f such that
 *                      1+x = 2^k * (1+f),
 *         where  sqrt(2)/2 < 1+f < sqrt(2) .
 *
 *      Note. If k=0, then f=x is exact. However, if k!=0, then f
 *      may not be representable exactly. In that case, a correction
 *      term is need. Let u=1+x rounded. Let c = (1+x)-u, then
 *      log(1+x) - log(u) ~ c/u. Thus, we proceed to compute log(u),
 *      and add back the correction term c/u.
 *      (Note: when x > 2**53, one can simply return log(x))
 *
 *   2. Approximation of log(1+f): See log.c
 *
 *   3. Finally, log1p(x) = k*ln2 + log(1+f) + c/u. See log.c
 *
 * Special cases:
 *      log1p(x) is NaN with signal if x < -1 (including -INF) ;
 *      log1p(+INF) is +INF; log1p(-1) is -INF with signal;
 *      log1p(NaN) is that NaN with no signal.
 *
 * Accuracy:
 *      according to an error analysis, the error is always less than
 *      1 ulp (unit in the last place).
 *
 * Constants:
 * The hexadecimal values are the intended ones for the following
 * constants. The decimal values may be used, provided that the
 * compiler will convert from decimal to binary accurately enough
 * to produce the hexadecimal values shown.
 *
 * Note: Assuming log() return accurate answer, the following
 *       algorithm can be used to compute log1p(x) to within a few ULP:
 *
 *              u = 1+x;
 *              if(u==1.0) return x ; else
 *                         return log(u)*(x/(u-1.0));
 *
 *       See HP-15C Advanced Functions Handbook, p.193.
*/

use crate::{Float64, Float32};

consts!{
const LN2_HI: Float64 = 6.93147180369123816490e-01; /* 3fe62e42 fee00000 */
const LN2_LO: Float64 = 1.90821492927058770002e-10; /* 3dea39ef 35793c76 */
const LG1: Float64 = 6.666666666666735130e-01; /* 3FE55555 55555593 */
const LG2: Float64 = 3.999999999940941908e-01; /* 3FD99999 9997FA04 */
const LG3: Float64 = 2.857142874366239149e-01; /* 3FD24924 94229359 */
const LG4: Float64 = 2.222219843214978396e-01; /* 3FCC71C5 1D8E78AF */
const LG5: Float64 = 1.818357216161805012e-01; /* 3FC74664 96CB03DE */
const LG6: Float64 = 1.531383769920937332e-01; /* 3FC39A09 D078C69F */
const LG7: Float64 = 1.479819860511658591e-01; /* 3FC2F112 DF3E5244 */
}

/// Return the natural logarithm of `1+x`.
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn log1p(x: Float64) -> Float64 {
    let mut ui: u64 = x.to_bits();
    let mut f: Float64 = 0.;
    let mut c: Float64 = 0.;
    let mut hu: u32;
    let mut k: i32;

    let hx: u32 = (ui >> 32) as u32;
    k = 1;
    if hx < 0x3fda827a || (hx >> 31) > 0 {
        /* 1+x < sqrt(2)+ */
        if hx >= 0xbff00000 {
            /* x <= -1.0 */
            if x == -1. {
                return x / 0.0; /* log1p(-1) = -inf */
            }
            return (x - x) / 0.0; /* log1p(x<-1) = NaN */
        }
        if hx << 1 < 0x3ca00000 << 1 {
            /* |x| < 2**-53 */
            /* underflow if subnormal */
            if (hx & 0x7ff00000) == 0 {
                force_eval!(x as Float32);
            }
            return x;
        }
        if hx <= 0xbfd2bec4 {
            /* sqrt(2)/2- <= 1+x < sqrt(2)+ */
            k = 0;
            c = 0.;
            f = x;
        }
    } else if hx >= 0x7ff00000 {
        return x;
    }
    if k > 0 {
        ui = (1. + x).to_bits();
        hu = (ui >> 32) as u32;
        hu += 0x3ff00000 - 0x3fe6a09e;
        k = (hu >> 20) as i32 - 0x3ff;
        /* correction term ~ log(1+x)-log(u), avoid underflow in c/u */
        if k < 54 {
            c = if k >= 2 {
                1. - (Float64::from_bits(ui) - x)
            } else {
                x - (Float64::from_bits(ui) - 1.)
            };
            c /= Float64::from_bits(ui);
        } else {
            c = 0.;
        }
        /* reduce u into [sqrt(2)/2, sqrt(2)] */
        hu = (hu & 0x000fffff) + 0x3fe6a09e;
        ui = (hu as u64) << 32 | (ui & 0xffffffff);
        f = Float64::from_bits(ui) - 1.;
    }
    let hfsq: Float64 = 0.5 * f * f;
    let s: Float64 = f / (2.0 + f);
    let z: Float64 = s * s;
    let w: Float64 = z * z;
    let t1: Float64 = w * (LG2 + w * (LG4 + w * LG6));
    let t2: Float64 = z * (LG1 + w * (LG3 + w * (LG5 + w * LG7)));
    let r: Float64 = t2 + t1;
    let dk: Float64 = k as Float64;
    s * (hfsq + r) + (dk * LN2_LO + c) - hfsq + f + dk * LN2_HI
}
