/* origin: FreeBSD /usr/src/lib/msun/src/e_lgamma_r.c */
/**
 * ====================================================
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 *
 * Developed at SunSoft, a Sun Microsystems, Inc. business.
 * Permission to use, copy, modify, and distribute this
 * software is freely granted, provided that this notice
 * is preserved.
 * ====================================================
 *
*/
/**
 * lgamma_r(x, signgamp)
 * Reentrant version of the logarithm of the Gamma function
 * with user provide pointer for the sign of Gamma(x).
 *
 * Method:
 *   1. Argument Reduction for 0 < x <= 8
 *      Since gamma(1+s)=s*gamma(s), for x in [0,8], we may
 *      reduce x to a number in [1.5,2.5] by
 *              lgamma(1+s) = log(s) + lgamma(s)
 *      for example,
 *              lgamma(7.3) = log(6.3) + lgamma(6.3)
 *                          = log(6.3*5.3) + lgamma(5.3)
 *                          = log(6.3*5.3*4.3*3.3*2.3) + lgamma(2.3)
 *   2. Polynomial approximation of lgamma around its
 *      minimun ymin=1.461632144968362245 to maintain monotonicity.
 *      On [ymin-0.23, ymin+0.27] (i.e., [1.23164,1.73163]), use
 *              Let z = x-ymin;
 *              lgamma(x) = -1.214862905358496078218 + z^2*poly(z)
 *      where
 *              poly(z) is a 14 degree polynomial.
 *   2. Rational approximation in the primary interval [2,3]
 *      We use the following approximation:
 *              s = x-2.0;
 *              lgamma(x) = 0.5*s + s*P(s)/Q(s)
 *      with accuracy
 *              |P/Q - (lgamma(x)-0.5s)| < 2**-61.71
 *      Our algorithms are based on the following observation
 *
 *                             zeta(2)-1    2    zeta(3)-1    3
 * lgamma(2+s) = s*(1-Euler) + --------- * s  -  --------- * s  + ...
 *                                 2                 3
 *
 *      where Euler = 0.5771... is the Euler constant, which is very
 *      close to 0.5.
 *
 *   3. For x>=8, we have
 *      lgamma(x)~(x-0.5)log(x)-x+0.5*log(2pi)+1/(12x)-1/(360x**3)+....
 *      (better formula:
 *         lgamma(x)~(x-0.5)*(log(x)-1)-.5*(log(2pi)-1) + ...)
 *      Let z = 1/x, then we approximation
 *              f(z) = lgamma(x) - (x-0.5)(log(x)-1)
 *      by
 *                                  3       5             11
 *              w = w0 + w1*z + w2*z  + w3*z  + ... + w6*z
 *      where
 *              |w - f(z)| < 2**-58.74
 *
 *   4. For negative x, since (G is gamma function)
 *              -x*G(-x)*G(x) = PI/sin(PI*x),
 *      we have
 *              G(x) = PI/(sin(PI*x)*(-x)*G(-x))
 *      since G(-x) is positive, sign(G(x)) = sign(sin(PI*x)) for x<0
 *      Hence, for x<0, signgam = sign(sin(PI*x)) and
 *              lgamma(x) = log(|Gamma(x)|)
 *                        = log(PI/(|x*sin(PI*x)|)) - lgamma(-x);
 *      Note: one should avoid compute PI*(-x) directly in the
 *            computation of sin(PI*(-x)).
 *
 *   5. Special Cases
 *              lgamma(2+s) ~ s*(1-Euler) for tiny s
 *              lgamma(1) = lgamma(2) = 0
 *              lgamma(x) ~ -log(|x|) for tiny x
 *              lgamma(0) = lgamma(neg.integer) = inf and raise divide-by-zero
 *              lgamma(inf) = inf
 *              lgamma(-inf) = inf (bug for bug compatible with C99!?)
 *
*/

use crate::Float64;

use super::{floor, k_cos, k_sin, log};

const PI: Float64 = 3.141_592_653_589_793; /* 0x400921FB, 0x54442D18 */
const A0: Float64 = 7.721_566_490_153_287e-2; /* 0x3FB3C467, 0xE37DB0C8 */
const A1: Float64 = 3.224_670_334_241_136e-1; /* 0x3FD4A34C, 0xC4A60FAD */
const A2: Float64 = 6.735_230_105_312_927e-2; /* 0x3FB13E00, 0x1A5562A7 */
const A3: Float64 = 2.058_080_843_251_673_3e-2; /* 0x3F951322, 0xAC92547B */
const A4: Float64 = 7.385_550_860_814_029e-3; /* 0x3F7E404F, 0xB68FEFE8 */
const A5: Float64 = 2.890_513_836_734_156_3e-3; /* 0x3F67ADD8, 0xCCB7926B */
const A6: Float64 = 1.192_707_631_833_620_7e-3; /* 0x3F538A94, 0x116F3F5D */
const A7: Float64 = 5.100_697_921_535_113e-4; /* 0x3F40B6C6, 0x89B99C00 */
const A8: Float64 = 2.208_627_907_139_083_9e-4; /* 0x3F2CF2EC, 0xED10E54D */
const A9: Float64 = 1.080_115_672_475_839_4e-4; /* 0x3F1C5088, 0x987DFB07 */
const A10: Float64 = 2.521_445_654_512_573_3e-5; /* 0x3EFA7074, 0x428CFA52 */
const A11: Float64 = 4.486_409_496_189_151_6e-5; /* 0x3F07858E, 0x90A45837 */
const TC: Float64 = 1.461_632_144_968_362_2; /* 0x3FF762D8, 0x6356BE3F */
const TF: Float64 = -1.214_862_905_358_496_1e-1; /* 0xBFBF19B9, 0xBCC38A42 */
/* tt = -(tail of TF) */
const TT: Float64 = -3.638_676_997_039_505e-18; /* 0xBC50C7CA, 0xA48A971F */
const T0: Float64 = 4.838_361_227_238_100_5e-1; /* 0x3FDEF72B, 0xC8EE38A2 */
const T1: Float64 = -1.475_877_229_945_939e-1; /* 0xBFC2E427, 0x8DC6C509 */
const T2: Float64 = 6.462_494_023_913_339e-2; /* 0x3FB08B42, 0x94D5419B */
const T3: Float64 = -3.278_854_107_598_596_5e-2; /* 0xBFA0C9A8, 0xDF35B713 */
const T4: Float64 = 1.797_067_508_118_204e-2; /* 0x3F9266E7, 0x970AF9EC */
const T5: Float64 = -1.031_422_412_983_414_4e-2; /* 0xBF851F9F, 0xBA91EC6A */
const T6: Float64 = 6.100_538_702_462_913e-3; /* 0x3F78FCE0, 0xE370E344 */
const T7: Float64 = -3.684_520_167_811_382_6e-3; /* 0xBF6E2EFF, 0xB3E914D7 */
const T8: Float64 = 2.259_647_809_006_124_7e-3; /* 0x3F6282D3, 0x2E15C915 */
const T9: Float64 = -1.403_464_699_892_328_4e-3; /* 0xBF56FE8E, 0xBF2D1AF1 */
const T10: Float64 = 8.810_818_824_376_54e-4; /* 0x3F4CDF0C, 0xEF61A8E9 */
const T11: Float64 = -5.385_953_053_567_405e-4; /* 0xBF41A610, 0x9C73E0EC */
const T12: Float64 = 3.156_320_709_036_259_5e-4; /* 0x3F34AF6D, 0x6C0EBBF7 */
const T13: Float64 = -3.127_541_683_751_208_6e-4; /* 0xBF347F24, 0xECC38C38 */
const T14: Float64 = 3.355_291_926_355_191e-4; /* 0x3F35FD3E, 0xE8C2D3F4 */
const U0: Float64 = -7.721_566_490_153_287e-2; /* 0xBFB3C467, 0xE37DB0C8 */
const U1: Float64 = 6.328_270_640_250_934e-1; /* 0x3FE4401E, 0x8B005DFF */
const U2: Float64 = 1.454_922_501_372_347_7; /* 0x3FF7475C, 0xD119BD6F */
const U3: Float64 = 9.777_175_279_633_727e-1; /* 0x3FEF4976, 0x44EA8450 */
const U4: Float64 = 2.289_637_280_646_924_5e-1; /* 0x3FCD4EAE, 0xF6010924 */
const U5: Float64 = 1.338_109_185_367_876_6e-2; /* 0x3F8B678B, 0xBF2BAB09 */
const V1: Float64 = 2.455_977_937_130_411_3; /* 0x4003A5D7, 0xC2BD619C */
const V2: Float64 = 2.128_489_763_798_934; /* 0x40010725, 0xA42B18F5 */
const V3: Float64 = 7.692_851_504_566_728e-1; /* 0x3FE89DFB, 0xE45050AF */
const V4: Float64 = 1.042_226_455_933_691_3e-1; /* 0x3FBAAE55, 0xD6537C88 */
const V5: Float64 = 3.217_092_422_824_239e-3; /* 0x3F6A5ABB, 0x57D0CF61 */
const S0: Float64 = -7.721_566_490_153_287e-2; /* 0xBFB3C467, 0xE37DB0C8 */
const S1: Float64 = 2.149_824_159_606_088_5e-1; /* 0x3FCB848B, 0x36E20878 */
const S2: Float64 = 3.257_787_964_089_31e-1; /* 0x3FD4D98F, 0x4F139F59 */
const S3: Float64 = 1.463_504_726_524_644_5e-1; /* 0x3FC2BB9C, 0xBEE5F2F7 */
const S4: Float64 = 2.664_227_030_336_386e-2; /* 0x3F9B481C, 0x7E939961 */
const S5: Float64 = 1.840_284_514_073_377_2e-3; /* 0x3F5E26B6, 0x7368F239 */
const S6: Float64 = 3.194_753_265_841_009e-5; /* 0x3F00BFEC, 0xDD17E945 */
const R1: Float64 = 1.392_005_334_676_210_5; /* 0x3FF645A7, 0x62C4AB74 */
const R2: Float64 = 7.219_355_475_671_381e-1; /* 0x3FE71A18, 0x93D3DCDC */
const R3: Float64 = 1.719_338_656_328_030_8e-1; /* 0x3FC601ED, 0xCCFBDF27 */
const R4: Float64 = 1.864_591_917_156_529e-2; /* 0x3F9317EA, 0x742ED475 */
const R5: Float64 = 7.779_424_963_818_936e-4; /* 0x3F497DDA, 0xCA41A95B */
const R6: Float64 = 7.326_684_307_446_256e-6; /* 0x3EDEBAF7, 0xA5B38140 */
const W0: Float64 = 4.189_385_332_046_727e-1; /* 0x3FDACFE3, 0x90C97D69 */
const W1: Float64 = 8.333_333_333_333_297e-2; /* 0x3FB55555, 0x5555553B */
const W2: Float64 = -2.777_777_777_287_755_4e-3; /* 0xBF66C16C, 0x16B02E5C */
const W3: Float64 = 7.936_505_586_430_196e-4; /* 0x3F4A019F, 0x98CF38B6 */
const W4: Float64 = -5.951_875_574_503_4e-4; /* 0xBF4380CB, 0x8C0FE741 */
const W5: Float64 = 8.363_399_189_962_821e-4; /* 0x3F4B67BA, 0x4CDAD5D1 */
const W6: Float64 = -1.630_929_340_965_752_7e-3; /* 0xBF5AB89D, 0x0B9E43E4 */

/* sin(PI*x) assuming x > 2^-100, if sin(PI*x)==0 the sign is arbitrary */
fn sin_pi(mut x: Float64) -> Float64 {
    let mut n: i32;

    /* spurious inexact if odd int */
    x = 2.0 * (x * 0.5 - floor(x * 0.5)); /* x mod 2.0 */

    n = (x * 4.0) as i32;
    n = div!(n + 1, 2);
    x -= (n as Float64) * 0.5;
    x *= PI;

    match n {
        1 => k_cos(x, 0.0),
        2 => k_sin(-x, 0.0, 0),
        3 => -k_cos(x, 0.0),
        0 | _ => k_sin(x, 0.0, 0),
    }
}

/// Natural logarithm of gamma function
/// 
/// Returns the natural logarithm of the absolute value of the gamma function of x,
/// and the sign of the gamma function of x
#[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
pub fn lgamma_r(mut x: Float64) -> (Float64, i32) {
    let u: u64 = x.to_bits();
    let mut t: Float64;
    let y: Float64;
    let mut z: Float64;
    let nadj: Float64;
    let p: Float64;
    let p1: Float64;
    let p2: Float64;
    let p3: Float64;
    let q: Float64;
    let mut r: Float64;
    let w: Float64;
    
    
    let i: i32;
    let mut signgam: i32;

    /* purge off +-inf, NaN, +-0, tiny and negative arguments */
    signgam = 1;
    let sign: bool = (u >> 63) != 0;
    let ix: u32 = ((u >> 32) as u32) & 0x7fffffff;
    if ix >= 0x7ff00000 {
        return (x * x, signgam);
    }
    if ix < (0x3ff - 70) << 20 {
        /* |x|<2**-70, return -log(|x|) */
        if sign {
            x = -x;
            signgam = -1;
        }
        return (-log(x), signgam);
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
        nadj = log(PI / (t * x));
    } else {
        nadj = 0.0;
    }

    /* purge off 1 and 2 */
    if (ix == 0x3ff00000 || ix == 0x40000000) && (u & 0xffffffff) == 0 {
        r = 0.0;
    }
    /* for x < 2.0 */
    else if ix < 0x40000000 {
        if ix <= 0x3feccccc {
            /* lgamma(x) = lgamma(x+1)-log(x) */
            r = -log(x);
            if ix >= 0x3FE76944 {
                y = 1.0 - x;
                i = 0;
            } else if ix >= 0x3FCDA661 {
                y = x - (TC - 1.0);
                i = 1;
            } else {
                y = x;
                i = 2;
            }
        } else {
            r = 0.0;
            if ix >= 0x3FFBB4C3 {
                /* [1.7316,2] */
                y = 2.0 - x;
                i = 0;
            } else if ix >= 0x3FF3B4C4 {
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
    } else if ix < 0x40200000 {
        /* x < 8.0 */
        i = x as i32;
        y = x - (i as Float64);
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
            r += log(z);
        }
    } else if ix < 0x43900000 {
        /* 8.0 <= x < 2**58 */
        t = log(x);
        z = 1.0 / x;
        y = z * z;
        w = W0 + z * (W1 + y * (W2 + y * (W3 + y * (W4 + y * (W5 + y * W6)))));
        r = (x - 0.5) * (t - 1.0) + w;
    } else {
        /* 2**58 <= x <= inf */
        r = x * (log(x) - 1.0);
    }
    if sign {
        r = nadj - r;
    }
    return (r, signgam);
}
