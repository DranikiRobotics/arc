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
    ($array:expr, $index:expr, = , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) = $rhs;
        }
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) += $rhs;
        }
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) -= $rhs;
        }
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        unsafe {
            *$array.get_unchecked_mut($index) &= $rhs;
        }
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        unsafe { *$array.get_unchecked_mut($index) == $rhs }
    };
}

#[cfg(debug_assertions)]
macro_rules! i {
    ($array:expr, $index:expr) => {
        *$array.get($index).unwrap()
    };
    ($array:expr, $index:expr, = , $rhs:expr) => {
        *$array.get_mut($index).unwrap() = $rhs;
    };
    ($array:expr, $index:expr, -= , $rhs:expr) => {
        *$array.get_mut($index).unwrap() -= $rhs;
    };
    ($array:expr, $index:expr, += , $rhs:expr) => {
        *$array.get_mut($index).unwrap() += $rhs;
    };
    ($array:expr, $index:expr, &= , $rhs:expr) => {
        *$array.get_mut($index).unwrap() &= $rhs;
    };
    ($array:expr, $index:expr, == , $rhs:expr) => {
        *$array.get_mut($index).unwrap() == $rhs
    };
}

// Temporary macro to avoid panic codegen for division (in debug mode too). At
// the time of this writing this is only used in a few places, and once
// rust-lang/rust#72751 is fixed then this macro will no longer be necessary and
// the native `/` operator can be used and panics won't be codegen'd.
#[cfg(debug_assertions)]
macro_rules! div {
    ($a:expr, $b:expr) => {
        $a / $b
    };
}

#[cfg(not(debug_assertions))]
macro_rules! div {
    ($a:expr, $b:expr) => {
        unsafe { core::intrinsics::unchecked_div($a, $b) }
    };
}

macro_rules! consts {
    (const $name:ident: $ty:ty = $value:expr; $($t:tt)* ) => (
        #[allow(clippy::excessive_precision)]
        #[deny(clippy::approx_constant)]
        const $name: $ty = $value;
        consts!($($t)*);
    );
    () => ();
}

macro_rules! llvm_intrinsically_optimized {
    (#[cfg($($clause:tt)*)] $e:expr) => {
        #[cfg(all($($clause)*))]
        { $e }
    };
}

macro_rules! import {
    ($p:vis $n:ident) => (
        mod $n; #[allow(unused_imports)] $p use self::$n::*;
    );
    ($p:vis $n:ident $(, $m:ident)*) => (
        import!($p $n); import!($p $($m),*);
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
import!(pub ldexp, ldexpf, lgamma, lgamma_r, lgammaf, lgammaf_r, ln, lnf, log, log10, log10f, log1p, log1pf, log2, log2f, logf);
import!(pub modf, modff);
import!(pub nextafter, nextafterf);
import!(pub pow, powf);
import!(pub remainder, remainderf, remquo, remquof, rint, rintf, round, roundf);
import!(pub scalbn, scalbnf, sin, sincos, sincosf, sinf, sinh, sinhf, sqrt, sqrtf);
import!(pub tan, tanf, tanh, tanhf, tgamma, tgammaf, trunc, truncf);

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

use crate::Float64;

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
