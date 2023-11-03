use crate::*;

mod number;
mod float;

pub use number::Number;
pub use float::Float;

macro_rules! simpl {
    ($n:ident => $m:ident $($o:tt)*) => (
        #[inline]
        #[must_use]
        fn $n(&self) -> Self {
            math::$m(*self)
        }
        simpl!($($o)*);
    );
    ($n:ident $($o:tt)*) => (
        simpl!($n => $n $($o)*);
    );
    () => ();
}

impl Float for crate::float {
    fn signum(&self) -> Self {
        todo!()
    }

    fn mul_add(&self, a: Self, b: Self) -> Self {
        todo!()
    }

    fn div_euclid(&self, rhs: Self) -> Self {
        todo!()
    }

    fn rem_euclid(&self, rhs: Self) -> Self {
        todo!()
    }

    fn powi(&self, n: int) -> Self {
        todo!()
    }

    fn powf(&self, n: Self) -> Self {
        todo!()
    }
    simpl!(floor ceil round trunc abs => fabs sqrt exp exp2);

    fn ln(&self) -> Self {
        todo!()
    }

    fn log(&self, base: Self) -> Self {
        todo!()
    }

    fn log2(&self) -> Self {
        todo!()
    }

    fn log10(&self) -> Self {
        todo!()
    }

    fn cbrt(&self) -> Self {
        todo!()
    }

    fn hypot(&self, other: Self) -> Self {
        todo!()
    }

    fn sin(&self) -> Self {
        todo!()
    }

    fn cos(&self) -> Self {
        todo!()
    }

    fn tan(&self) -> Self {
        todo!()
    }

    fn asin(&self) -> Self {
        todo!()
    }

    fn acos(&self) -> Self {
        todo!()
    }

    fn atan(&self) -> Self {
        todo!()
    }

    fn atan2(&self, other: Self) -> Self {
        todo!()
    }

    fn sin_cos(&self) -> (Self, Self) {
        todo!()
    }

    fn exp_m1(&self) -> Self {
        todo!()
    }

    fn ln_1p(&self) -> Self {
        todo!()
    }

    fn sinh(&self) -> Self {
        todo!()
    }

    fn cosh(&self) -> Self {
        todo!()
    }

    fn tanh(&self) -> Self {
        todo!()
    }

    fn asinh(&self) -> Self {
        todo!()
    }

    fn acosh(&self) -> Self {
        todo!()
    }

    fn atanh(&self) -> Self {
        todo!()
    }
}