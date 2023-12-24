macro_rules! force_eval {
    ($e:expr) => {
        unsafe { ::core::ptr::read_volatile(&$e) }
    };
}

#[cfg(not(debug_assertions))]
macro_rules! i {
    ($array:expr, $index:expr) => {
        unsafe { *$array.get_unchecked($index) }
    };
    ($array:expr, $index:expr, $t:tt, $rhs:expr) => {
        unsafe { *$array.get_unchecked_mut($index) $t $rhs }
    };
}

#[cfg(debug_assertions)]
macro_rules! i {
    ($array:expr, $index:expr) => {
        *$array.get($index).unwrap()
    };
    ($array:expr, $index:expr, $t:tt, $rhs:expr) => {
        *$array.get_mut($index).unwrap() $t $rhs
    };
}

// Temporary macro to avoid panic codegen for division (in debug mode too). At
// the time of this writing this is only used in a few places, and once
// rust-lang/rust#72751 is fixed then this macro will no longer be necessary and
// the native `/` operator can be used and panics won't be codegen'd.
#[cfg(any(debug_assertions, not(feature = "unstable")))]
macro_rules! div {
    ($a:expr, $b:expr) => {
        $a / $b
    };
}

#[cfg(all(not(debug_assertions), feature = "unstable"))]
macro_rules! div {
    ($a:expr, $b:expr) => {
        unsafe { core::intrinsics::unchecked_div($a, $b) }
    };
}

macro_rules! llvm_intrinsically_optimized {
    (#[cfg($($clause:tt)*)] $e:expr) => {
        #[cfg(all(not(debug_assertions), feature = "unstable", $($clause)*))]
        {
            if true { // thwart the dead code lint
                $e
            }
        }
    };
}

macro_rules! tup {
    ($name:ident $($k:ident:$v:ty),+) => (
        #[repr(C)]
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        pub struct $name {
            $( pub $k: $v ),+
        }
        impl $name {
            #[inline(always)]
            #[must_use = "this is a constructor"]
            pub const fn from(($($k),+): ($($v),+)) -> Self {
                Self { $($k),+ }
            }
        }
    );
}

use crate::{Float32, Float64, Int};

tup!(Tuple_Float64_Int f: Float64, i: Int);
tup!(Tuple_Float32_Int f: Float32, i: Int);
tup!(Tuple_Float64_Float64 f1: Float64, f2: Float64);
tup!(Tuple_Float32_Float32 f1: Float32, f2: Float32);

macro_rules! consts {
    (const $name:ident: $ty:ty = $value:expr; $($t:tt)* ) => (
        #[allow(clippy::excessive_precision)]
        #[deny(clippy::approx_constant)]
        const $name: $ty = $value;
        consts!($($t)*);
    );
    () => ();
}

macro_rules! import {
    ($p:vis $($m:ident),+) => (
        $(mod $m; #[allow(unused_imports)] $p use self::$m::*;)+
    );
}

// Public modules
import!(pub acos, acosf, acosh, acoshf, asin, asinf, asinh, asinhf, atan, atan2, atan2f, atanf, atanh, atanhf);
import!(pub cbrt, cbrtf, ceil, ceilf, copysign, copysignf, cos, cosf, cosh, coshf);
import!(pub erf, erff, exp, exp10, exp10f, exp2, exp2f, expf, expm1, expm1f);
import!(pub fabs, fabsf, fdim, fdimf, floor, floorf, fma, fmaf, fmax, fmaxf, fmin, fminf, fmod, fmodf, frexp, frexpf);
import!(pub hypot, hypotf);
import!(pub ilogb, ilogbf);
import!(pub j0, j0f, j1, j1f, jn, jnf);
import!(pub ldexp, ldexpf, lgamma, lgamma_r, lgammaf, lgammaf_r, ln, lnf, log, log10, log10f, ln1p, ln1pf, log2, log2f, logf);
import!(pub modf, modff);
import!(pub nextafter, nextafterf);
import!(pub pow, powf);
import!(pub remainder, remainderf, remquo, remquof, rint, rintf, round, roundf);
import!(pub scalbn, scalbnf, sin, sincos, sincosf, sinf, sinh, sinhf, sqrt, sqrtf);
import!(pub tan, tanf, tanh, tanhf, tgamma, tgammaf, trunc, truncf);
import!(pub ulp);

// Private modules
import!(
    expo2,
    fenv,
    k_cos,
    k_cosf,
    k_expo2,
    k_expo2f,
    k_sin,
    k_sinf,
    k_tan,
    k_tanf,
    rem_pio2,
    rem_pio2_large,
    rem_pio2f
);

#[inline]
fn get_high_word(x: Float64) -> u32 {
    (x.to_bits() >> 32) as u32
}

#[inline]
fn get_low_word(x: Float64) -> u32 {
    x.to_bits() as u32
}

#[inline]
fn with_set_high_word(f: Float64, hi: u32) -> Float64 {
    let mut tmp = f.to_bits();
    tmp &= 0x00000000_ffffffff;
    tmp |= (hi as u64) << 32;
    Float64::from_bits(tmp)
}

#[inline]
fn with_set_low_word(f: Float64, lo: u32) -> Float64 {
    let mut tmp = f.to_bits();
    tmp &= 0xffffffff_00000000;
    tmp |= lo as u64;
    Float64::from_bits(tmp)
}

#[inline]
fn combine_words(hi: u32, lo: u32) -> Float64 {
    Float64::from_bits((hi as u64) << 32 | lo as u64)
}
