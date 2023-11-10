/* origin: FreeBSD /usr/src/lib/msun/src/e_j1f.c */
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

use super::{cosf, fabsf, logf, sinf, sqrtf};

consts!{
const INVSQRTPI: Float32 = 5.6418961287e-01; /* 0x3f106ebb */
const TPI: Float32 = 6.3661974669e-01; /* 0x3f22f983 */
}

fn common(ix: u32, x: Float32, y1: bool, sign: bool) -> Float32 {
    let mut s = sinf(x) as Float64;
    if y1 {
        s = -s;
    }
    let c = cosf(x) as Float64;
    let mut cc = s - c;
    if ix < 0x7f000000 {
        let mut ss = -s - c;
        let z = cosf(2.0 * x) as Float64;
        if s * c > 0.0 {
            cc = z / ss;
        } else {
            ss = z / cc;
        }
        if ix < 0x58800000 {
            if y1 {
                ss = -ss;
            }
            cc = (ponef(x) as Float64) * cc - (qonef(x) as Float64) * ss;
        }
    }
    if sign {
        cc = -cc;
    }
    return (((INVSQRTPI as Float64) * cc) / (sqrtf(x) as Float64)) as Float32;
}

/* R0/S0 on [0,2] */
consts!{
const R00: Float32 = -6.2500000000e-02; /* 0xbd800000 */
const R01: Float32 = 1.4070566976e-03; /* 0x3ab86cfd */
const R02: Float32 = -1.5995563444e-05; /* 0xb7862e36 */
const R03: Float32 = 4.9672799207e-08; /* 0x335557d2 */
const S01: Float32 = 1.9153760746e-02; /* 0x3c9ce859 */
const S02: Float32 = 1.8594678841e-04; /* 0x3942fab6 */
const S03: Float32 = 1.1771846857e-06; /* 0x359dffc2 */
const S04: Float32 = 5.0463624390e-09; /* 0x31ad6446 */
const S05: Float32 = 1.2354227016e-11; /* 0x2d59567e */
}

/// Bessel function of the first kind of order one
/// 
/// Calculates the Bessel function of the first kind of order one of `x`.
pub fn j1f(x: Float32) -> Float32 {
    let mut z: Float32;
    let mut ix = x.to_bits();
    let sign = (ix >> 31) != 0;
    ix &= 0x7fffffff;
    if ix >= 0x7f800000 {
        return 1.0 / (x * x);
    }
    if ix >= 0x40000000 {
        /* |x| >= 2 */
        return common(ix, fabsf(x), false, sign);
    }
    if ix >= 0x39000000 {
        /* |x| >= 2**-13 */
        z = x * x;
        let r = z * (R00 + z * (R01 + z * (R02 + z * R03)));
        let s = 1.0 + z * (S01 + z * (S02 + z * (S03 + z * (S04 + z * S05))));
        z = 0.5 + r / s;
    } else {
        z = 0.5;
    }
    return z * x;
}

consts!{
const U0: [Float32; 5] = [
    -1.9605709612e-01, /* 0xbe48c331 */
    5.0443872809e-02,  /* 0x3d4e9e3c */
    -1.9125689287e-03, /* 0xbafaaf2a */
    2.3525259166e-05,  /* 0x37c5581c */
    -9.1909917899e-08, /* 0xb3c56003 */
];
const V0: [Float32; 5] = [
    1.9916731864e-02, /* 0x3ca3286a */
    2.0255257550e-04, /* 0x3954644b */
    1.3560879779e-06, /* 0x35b602d4 */
    6.2274145840e-09, /* 0x31d5f8eb */
    1.6655924903e-11, /* 0x2d9281cf */
];
}

/// Bessel function of the second kind of order one
/// 
/// Calculates the Bessel function of the second kind of order one of `x`.
pub fn y1f(x: Float32) -> Float32 {
    let ix = x.to_bits();
    if (ix & 0x7fffffff) == 0 {
        return -1.0 / 0.0;
    }
    if (ix >> 31) != 0 {
        return 0.0 / 0.0;
    }
    if ix >= 0x7f800000 {
        return 1.0 / x;
    }
    if ix >= 0x40000000 {
        /* |x| >= 2.0 */
        return common(ix, x, true, false);
    }
    if ix < 0x33000000 {
        /* x < 2**-25 */
        return -TPI / x;
    }
    let z = x * x;
    let u = U0[0] + z * (U0[1] + z * (U0[2] + z * (U0[3] + z * U0[4])));
    let v = 1.0 + z * (V0[0] + z * (V0[1] + z * (V0[2] + z * (V0[3] + z * V0[4]))));
    return x * (u / v) + TPI * (j1f(x) * logf(x) - 1.0 / x);
}

/**
 * For x >= 8, the asymptotic expansions of pone is
 *      1 + 15/128 s^2 - 4725/2^15 s^4 - ...,   where s = 1/x.
 * We approximate pone by
 *      pone(x) = 1 + (R/S)
 * where  R = pr0 + pr1*s^2 + pr2*s^4 + ... + pr5*s^10
 *        S = 1 + ps0*s^2 + ... + ps4*s^10
 * and
 *      | pone(x)-1-R/S | <= 2  ** ( -60.06)
*/
const PR8: [Float32; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.0000000000e+00, /* 0x00000000 */
    1.171_875e-1, /* 0x3df00000 */
    1.323_948_1e1, /* 0x4153d4ea */
    4.120_518_5e2, /* 0x43ce06a3 */
    3.874_745_4e3, /* 0x45722bed */
    7.914_479_5e3, /* 0x45f753d6 */
];
const PS8: [Float32; 5] = [
    1.142_073_7e2, /* 0x42e46a2c */
    3.650_931e3, /* 0x45642ee5 */
    3.695_620_7e4, /* 0x47105c35 */
    9.760_28e4, /* 0x47bea166 */
    3.080_427_1e4, /* 0x46f0a88b */
];

const PR5: [Float32; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    1.319_905_2e-11, /* 0x2d68333f */
    1.171_874_9e-1, /* 0x3defffff */
    6.802_751, /* 0x40d9b023 */
    1.083_081_8e2, /* 0x42d89dca */
    5.176_361_7e2, /* 0x440168b7 */
    5.287_152e2, /* 0x44042dc6 */
];
const PS5: [Float32; 5] = [
    5.928_059_8e1, /* 0x426d1f55 */
    9.914_014e2, /* 0x4477d9b1 */
    5.353_267e3, /* 0x45a74a23 */
    7.844_690_4e3, /* 0x45f52586 */
    1.504_046_9e3, /* 0x44bc0180 */
];

const PR3: [Float32; 6] = [
    3.025_039e-9, /* 0x314fe10d */
    1.171_868_7e-1, /* 0x3defffab */
    3.932_977_4, /* 0x407bb5e7 */
    3.511_940_4e1, /* 0x420c7a45 */
    9.105_501e1, /* 0x42b61c2a */
    4.855_906_7e1, /* 0x42423c7c */
];
const PS3: [Float32; 5] = [
    3.479_131e1, /* 0x420b2a4d */
    3.367_624_5e2, /* 0x43a86198 */
    1.046_871_5e3, /* 0x4482dbe3 */
    8.908_113_4e2, /* 0x445eb3ed */
    1.037_879_3e2, /* 0x42cf936c */
];

const PR2: [Float32; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    1.077_108_3e-7, /* 0x33e74ea8 */
    1.171_762_2e-1, /* 0x3deffa16 */
    2.368_515, /* 0x401795c0 */
    1.224_261_1e1, /* 0x4143e1bc */
    1.769_397_2e1, /* 0x418d8d41 */
    5.073_523, /* 0x40a25a4d */
];
const PS2: [Float32; 5] = [
    2.143_648_5e1, /* 0x41ab7dec */
    1.252_902_3e2, /* 0x42fa9499 */
    2.322_764_7e2, /* 0x436846c7 */
    1.176_793_75e2, /* 0x42eb5bd7 */
    8.364_639, /* 0x4105d590 */
];

fn ponef(x: Float32) -> Float32 {
    let p: &[Float32; 6];
    let q: &[Float32; 5];
    
    
    
    let mut ix: u32;

    ix = x.to_bits();
    ix &= 0x7fffffff;
    if ix >= 0x41000000 {
        p = &PR8;
        q = &PS8;
    } else if ix >= 0x409173eb {
        p = &PR5;
        q = &PS5;
    } else if ix >= 0x4036d917 {
        p = &PR3;
        q = &PS3;
    } else
    /*ix >= 0x40000000*/
    {
        p = &PR2;
        q = &PS2;
    }
    let z: Float32 = 1.0 / (x * x);
    let r: Float32 = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    let s: Float32 = 1.0 + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * q[4]))));
    return 1.0 + r / s;
}

/**
 * For x >= 8, the asymptotic expansions of qone is
 *      3/8 s - 105/1024 s^3 - ..., where s = 1/x.
 * We approximate pone by
 *      qone(x) = s*(0.375 + (R/S))
 * where  R = qr1*s^2 + qr2*s^4 + ... + qr5*s^10
 *        S = 1 + qs1*s^2 + ... + qs6*s^12
 * and
 *      | qone(x)/s -0.375-R/S | <= 2  ** ( -61.13)
*/

const QR8: [Float32; 6] = [
    /* for x in [inf, 8]=1/[0,0.125] */
    0.0000000000e+00,  /* 0x00000000 */
    -1.025_390_6e-1, /* 0xbdd20000 */
    -1.627_175_3e1, /* 0xc1822c8d */
    -7.596_017_5e2, /* 0xc43de683 */
    -1.184_980_7e4, /* 0xc639273a */
    -4.843_851e4, /* 0xc73d3683 */
];
const QS8: [Float32; 6] = [
    1.613_953_7e2,  /* 0x43216537 */
    7.825_386e3,  /* 0x45f48b17 */
    1.338_753_4e5,  /* 0x4802bcd6 */
    7.196_577_5e5,  /* 0x492fb29c */
    6.666_012_5e5,  /* 0x4922be94 */
    -2.944_902_5e5, /* 0xc88fcb48 */
];

const QR5: [Float32; 6] = [
    /* for x in [8,4.5454]=1/[0.125,0.22001] */
    -2.089_799_3e-11, /* 0xadb7d219 */
    -1.025_390_5e-1, /* 0xbdd1fffe */
    -8.056_448, /* 0xc100e736 */
    -1.836_696e2, /* 0xc337ab6b */
    -1.373_193_7e3, /* 0xc4aba633 */
    -2.612_444_3e3, /* 0xc523471c */
];
const QS5: [Float32; 6] = [
    8.127_655e1,  /* 0x42a28d98 */
    1.991_798_7e3,  /* 0x44f8f98f */
    1.746_848_4e4,  /* 0x468878f8 */
    4.985_142_6e4,  /* 0x4742bb6d */
    2.794_807_4e4,  /* 0x46da5826 */
    -4.719_183_6e3, /* 0xc5937978 */
];

const QR3: [Float32; 6] = [
    -5.078_312_4e-9, /* 0xb1ae7d4f */
    -1.025_378_3e-1, /* 0xbdd1ff5b */
    -4.610_116, /* 0xc0938612 */
    -5.784_722e1, /* 0xc267638e */
    -2.282_445_4e2, /* 0xc3643e9a */
    -2.192_101_3e2, /* 0xc35b35cb */
];
const QS3: [Float32; 6] = [
    4.766_515_4e1,  /* 0x423ea91e */
    6.738_651e2,  /* 0x4428775e */
    3.380_152_8e3,  /* 0x45534272 */
    5.547_729e3,  /* 0x45ad5dd5 */
    1.903_119_1e3,  /* 0x44ede3d0 */
    -1.352_011_9e2, /* 0xc3073381 */
];

const QR2: [Float32; 6] = [
    /* for x in [2.8570,2]=1/[0.3499,0.5] */
    -1.783_817_3e-7, /* 0xb43f8932 */
    -1.025_170_46e-1, /* 0xbdd1f475 */
    -2.752_205_6, /* 0xc0302423 */
    -1.966_361_6e1, /* 0xc19d4f16 */
    -4.232_531_4e1, /* 0xc2294d1f */
    -2.137_192_2e1, /* 0xc1aaf9b2 */
];
const QS2: [Float32; 6] = [
    2.953_336_3e1,  /* 0x41ec4454 */
    2.529_815_5e2,  /* 0x437cfb47 */
    7.575_028e2,  /* 0x443d602e */
    7.393_932e2,  /* 0x4438d92a */
    1.559_49e2,  /* 0x431bf2f2 */
    -4.959_499, /* 0xc09eb437 */
];

fn qonef(x: Float32) -> Float32 {
    let p: &[Float32; 6];
    let q: &[Float32; 6];
    
    
    
    let mut ix: u32;

    ix = x.to_bits();
    ix &= 0x7fffffff;
    if ix >= 0x41000000 {
        p = &QR8;
        q = &QS8;
    } else if ix >= 0x409173eb {
        p = &QR5;
        q = &QS5;
    } else if ix >= 0x4036d917 {
        p = &QR3;
        q = &QS3;
    } else
    /*ix >= 0x40000000*/
    {
        p = &QR2;
        q = &QS2;
    }
    let z: Float32 = 1.0 / (x * x);
    let r: Float32 = p[0] + z * (p[1] + z * (p[2] + z * (p[3] + z * (p[4] + z * p[5]))));
    let s: Float32 = 1.0 + z * (q[0] + z * (q[1] + z * (q[2] + z * (q[3] + z * (q[4] + z * q[5])))));
    return (0.375 + r / s) / x;
}

// PowerPC tests are failing on LLVM 13: https://github.com/rust-lang/rust/issues/88520
#[cfg(not(target_arch = "powerpc64"))]
#[cfg(test)]
mod tests {
    use super::{j1f, y1f};
    #[test]
    fn test_j1f_2488() {
        // 0x401F3E49
        assert_eq!(j1f(2.4881766_f32), 0.49999475_f32);
    }
    #[test]
    fn test_y1f_2002() {
        //allow slightly different result on x87
        let res = y1f(2.0000002_f32);
        if cfg!(all(target_arch = "x86", not(target_feature = "sse2"))) && (res == -0.10703231_f32)
        {
            return;
        }
        assert_eq!(res, -0.10703229_f32);
    }
}
