use crate::*;
use super::*;

macro_rules! n { ($($o:tt)+) => ($(impl Number for $o{})+); }

n!(u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize Float32 Float64);

macro_rules! simpl {
    ($n:ident => $m:ident $($t:tt)*) => (
        #[inline]
        #[must_use]
        fn $n(&self) -> Self {
            l2math::$m(*self)
        }
        simpl!($($t)*);
    );
    ($n:ident $($t:tt)*) => (simpl!($n => $n $($t)*););
    () => ();
}

impl Sin for Float64 { simpl!(sin); }
impl Cos for Float64 { simpl!(cos); }
impl Sqrt for Float64 { simpl!(sqrt); }

#[allow(unused)]
impl Float for Float64 {
    simpl!(floor ceil round trunc abs => fabs exp exp2 ln log2 log10 cbrt
        tan asin acos atan exp_m1 => expm1 ln_1p => log1p
        sinh cosh tanh asinh acosh atanh);
    #[inline]
    #[must_use]
    fn mul_add(&self, a: Self, b: Self) -> Self {
        l2math::fma(*self, a, b)
    }
    #[inline]
    #[must_use]
    fn div_euclid(&self, rhs: Self) -> Self {
        todo!("div_euclid")
    }
    #[inline]
    #[must_use]
    fn rem_euclid(&self, rhs: Self) -> Self {
        todo!("rem_euclid")
    }
    #[inline]
    #[must_use]
    fn signum(&self) -> Self {
        if *self > 0.0 {
            1.0
        } else if *self < 0.0 {
            -1.0
        } else {
            Float64::NAN
        }
    }
    #[inline]
    #[must_use]
    fn signof(&self, rhs: Self) -> Self {
        self.abs() * rhs.signum()
    }
    #[inline]
    #[must_use]
    fn powi(&self, n: Int) -> Self {
        l2math::pow(*self, n as Float64)
    }
    #[inline]
    #[must_use]
    fn powf(&self, n: Self) -> Self {
        l2math::pow(*self, n)
    }
    #[inline]
    #[must_use]
    fn log(&self, base: Self) -> Self {
        l2math::log(*self) / l2math::log(base)
    }
    #[inline]
    #[must_use]
    fn hypot(&self, other: Self) -> Self {
        l2math::hypot(*self, other)
    }
    #[inline]
    #[must_use]
    fn atan2(&self, other: Self) -> Self {
        l2math::atan2(*self, other)
    }
    #[inline]
    #[must_use]
    fn fract(&self) -> Self {
        *self - self.floor()
    }
}

impl Sin for Float32 { simpl!(sin => sinf); }
impl Cos for Float32 { simpl!(cos => cosf); }
impl Sqrt for Float32 { simpl!(sqrt => sqrtf); }

impl Float for Float32 {
    simpl!(floor => floorf ceil => ceilf round => roundf trunc => truncf
        abs => fabsf exp => expf exp2 => exp2f ln => lnf log2 => log2f
        log10 => log10f cbrt => cbrtf tan => tanf asin => asinf
        acos => acosf atan => atanf exp_m1 => expm1f ln_1p => log1pf
        sinh => sinhf cosh => coshf tanh => tanhf asinh => asinhf
        acosh => acoshf atanh => atanhf);
    #[inline]
    #[must_use]
    fn mul_add(&self, a: Self, b: Self) -> Self {
        l2math::fmaf(*self, a, b)
    }
    #[inline]
    #[must_use]
    #[allow(unused)]
    fn div_euclid(&self, rhs: Self) -> Self {
        todo!("div_euclid")
    }
    #[inline]
    #[must_use]
    #[allow(unused)]
    fn rem_euclid(&self, rhs: Self) -> Self {
        todo!("rem_euclid")
    }
    #[inline]
    #[must_use]
    fn signum(&self) -> Self {
        if *self > 0.0 {
            1.0
        } else if *self < 0.0 {
            -1.0
        } else {
            Float32::NAN
        }
    }
    #[inline]
    #[must_use]
    fn signof(&self, rhs: Self) -> Self {
        self.abs() * rhs.signum()
    }
    #[inline]
    #[must_use]
    fn powi(&self, n: Int) -> Self {
        l2math::powf(*self, n as Float32)
    }
    #[inline]
    #[must_use]
    fn powf(&self, n: Self) -> Self {
        l2math::powf(*self, n)
    }
    #[inline]
    #[must_use]
    fn log(&self, base: Self) -> Self {
        l2math::logf(*self) / l2math::logf(base)
    }
    #[inline]
    #[must_use]
    fn hypot(&self, other: Self) -> Self {
        l2math::hypotf(*self, other)
    }
    #[inline]
    #[must_use]
    fn atan2(&self, other: Self) -> Self {
        l2math::atan2f(*self, other)
    }
    #[inline]
    #[must_use]
    fn fract(&self) -> Self {
        *self - self.floor()
    }
}
