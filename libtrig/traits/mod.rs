use crate::*;

mod float;
mod number;

pub use float::Float;
pub use number::Number;

macro_rules! simpl {
    ($n:ident => $m:ident $($t:tt)*) => (
        #[inline]
        #[must_use]
        fn $n(&self) -> Self {
            llm::$m(*self)
        }
        simpl!($($t)*);
    );
    ($n:ident $($t:tt)*) => (simpl!($n => $n $($t)*););
    () => ();
}

#[allow(unused)]
impl Float for Float64 {
    simpl!(floor ceil round trunc abs => fabs sqrt exp exp2 ln log2 log10 cbrt
        sin cos tan asin acos atan exp_m1 => expm1 ln_1p => log1p
        sinh cosh tanh asinh acosh atanh);
    #[inline]
    #[must_use]
    fn mul_add(&self, a: Self, b: Self) -> Self {
        llm::fma(*self, a, b)
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
    fn powi(&self, n: Int) -> Self {
        llm::pow(*self, n as Float64)
    }
    #[inline]
    #[must_use]
    fn powf(&self, n: Self) -> Self {
        llm::pow(*self, n)
    }
    #[inline]
    #[must_use]
    fn log(&self, base: Self) -> Self {
        llm::log(*self) / llm::log(base)
    }
    #[inline]
    #[must_use]
    fn hypot(&self, other: Self) -> Self {
        llm::hypot(*self, other)
    }
    #[inline]
    #[must_use]
    fn atan2(&self, other: Self) -> Self {
        llm::atan2(*self, other)
    }
    #[inline]
    #[must_use]
    fn fract(&self) -> Self {
        *self - self.floor()
    }
}
