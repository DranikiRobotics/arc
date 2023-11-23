use core::mem::zeroed;

use l2math::Float64;
use libtrig::prelude::*;

/**
 * [Dual number](https://en.wikipedia.org/wiki/Dual_number) to implement forward autodifferentiation.
 *
 * @param Param x
 * @property values (u, du/dx, d^2u/dx^2, ..., d^(n - 1)u/dx^(n - 1))
 */
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct DualNum<T: Float>(alloc::vec::Vec<T>);

impl<T: Float> DualNum<T> {
    pub const fn new(values: alloc::vec::Vec<T>) -> Self {
        Self(values)
    }

    /**
     * Makes dual number (c, 0, ..., 0) with size n representing a constant function c.
     *
     * @param Param x
     * @param c c
     */
    pub fn constant(c: T, n: usize) -> Self {
        let values = (0..n)
            .map(|i| {
                if i == 0 {
                    return c;
                }
                T::from(libtrig::u2::ZERO)
            })
            .collect();
        Self::new(values)
    }

    /**
     * Makes dual number (x0, 1, 0, ..., 0) with size n representing a variable function x at x = x0.
     *
     * @param Param x
     * @param x0 x0
     */
    pub fn variable(x0: T, n: usize) -> Self {
        let values = (0..n)
            .map(|i| {
                if i == 0 {
                    return x0;
                }
                T::from(if i == 1 {
                    libtrig::u2::ONE
                } else {
                    libtrig::u2::ZERO
                })
            })
            .collect();
        Self::new(values)
    }

    pub fn cons(x: T, d: &DualNum<T>) -> Self {
        let values = core::iter::once(x).chain(d.0.iter().cloned()).collect();
        Self::new(values)
    }
    //
    //     /**
    //      * n
    //      */
    //     pub const fn size(&self) -> usize {
    //         self.values.len()
    //     }
    //
    //     pub const fn value(&self) -> Float64 {
    //         self.values[0]
    //     }
    //
    //     pub const fn get(&self, i: usize) -> Float64 {
    //         self.values[i]
    //     }
    //
    //     pub const fn values(&self) -> alloc::vec::Vec<Float64> {
    //         self.values.clone()
    //     }
    //
    //     pub const fn drop(&self, n: usize) -> Self {
    //         Self::new(self.values[n..].to_vec())
    //     }
}

// impl<Param> std::ops::Add<&DualNum<Param>> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn add(self, d: &DualNum<Param>) -> DualNum<Param> {
//         let size = min(self.size(), d.size());
//         let mut out = DualNum::new(vec![0.0; size]);
//         for i in 0..size {
//             out.values[i] = self.values[i] + d.values[i];
//         }
//         out
//     }
// }
//
// impl<Param> std::ops::Sub<&DualNum<Param>> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn sub(self, d: &DualNum<Param>) -> DualNum<Param> {
//         let size = min(self.size(), d.size());
//         let mut out = DualNum::new(vec![0.0; size]);
//         for i in 0..size {
//             out.values[i] = self.values[i] - d.values[i];
//         }
//         out
//     }
// }
//
// impl<Param> std::ops::Mul<&DualNum<Param>> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn mul(self, d: &DualNum<Param>) -> DualNum<Param> {
//         let size = min(self.size(), d.size());
//         let mut out = DualNum::new(vec![0.0; size]);
//         if out.values.is_empty() {
//             return out;
//         }
//         out.values[0] = self.values[0] * d.values[0];
//         if out.size() == 1 {
//             return out;
//         }
//         out.values[1] = self.values[0] * d.values[1] + self.values[1] * d.values[0];
//         if out.size() == 2 {
//             return out;
//         }
//         out.values[2] = self.values[0] * d.values[2] + self.values[2] * d.values[0] +
//             2.0 * self.values[1] * d.values[1];
//         if out.size() == 3 {
//             return out;
//         }
//         out.values[3] = self.values[0] * d.values[3] + self.values[3] * d.values[0] +
//             3.0 * (self.values[2] * d.values[1] + self.values[1] * d.values[2]);
//         out
//     }
// }
//
// impl<Param> std::ops::Neg for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn neg(self) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         for i in 0..out.size() {
//             out.values[i] = -self.values[i];
//         }
//         out
//     }
// }
//
// impl<Param> DualNum<Param> {
//     fn recip(&self) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         if out.values.is_empty() {
//             return out;
//         }
//         let recip = 1.0 / self.values[0];
//         out.values[0] = recip;
//         if out.size() == 1 {
//             return out;
//         }
//         let neg_recip = -recip;
//         let neg_recip2 = recip * neg_recip;
//         let deriv = neg_recip2 * self.values[1];
//         out.values[1] = deriv;
//         if out.size() == 2 {
//             return out;
//         }
//         let int1 = 2.0 * neg_recip * deriv;
//         let deriv2 = int1 * self.values[1] + neg_recip2 * self.values[2];
//         out.values[2] = deriv2;
//         if out.size() == 3 {
//             return out;
//         }
//         let int2 = int1 * self.values[2];
//         out.values[3] =
//             int2 + neg_recip2 * self.values[3] +
//             int2 - 2.0 * (deriv * deriv + recip * deriv2) * self.values[1];
//         out
//     }
// }
//
// impl<Param> std::ops::Div<&DualNum<Param>> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn div(self, d: &DualNum<Param>) -> DualNum<Param> {
//         self * d.recip()
//     }
// }
//
// impl<Param> DualNum<Param> {
//     fn sqrt(&self) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         if out.values.is_empty() {
//             return out;
//         }
//         let sqrt = self.values[0].sqrt();
//         out.values[0] = sqrt;
//         if out.size() == 1 {
//             return out;
//         }
//         let recip = 1.0 / (2.0 * sqrt);
//         let deriv = recip * self.values[1];
//         out.values[1] = deriv;
//         if out.size() == 2 {
//             return out;
//         }
//         let neg_recip = -2.0 * recip;
//         let neg_recip2 = recip * neg_recip;
//         let int1 = neg_recip2 * deriv;
//         let second_deriv = int1 * self.values[1] + recip * self.values[2];
//         out.values[2] = second_deriv;
//         if out.size() == 3 {
//             return out;
//         }
//         let int2 = 2.0 * int1;
//         out.values[3] = recip * self.values[3] + int2 * self.values[2] +
//             (deriv * neg_recip * int2 + neg_recip2 * second_deriv) * self.values[1];
//         out
//     }
// }
//
// impl<Param> DualNum<Param> {
//     fn sin(&self) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         if out.values.is_empty() {
//             return out;
//         }
//         let sin = self.values[0].sin();
//         out.values[0] = sin;
//         if out.size() == 1 {
//             return out;
//         }
//         let cos = self.values[0].cos();
//         let deriv = cos * self.values[1];
//         out.values[1] = deriv;
//         if out.size() == 2 {
//             return out;
//         }
//         let in_deriv2 = self.values[1] * self.values[1];
//         out.values[2] = cos * self.values[2] - sin * in_deriv2;
//         if out.size() == 3 {
//             return out;
//         }
//         out.values[3] = cos * self.values[3] -
//             3.0 * sin * self.values[1] * self.values[2] -
//             deriv * in_deriv2;
//         out
//     }
// }
//
// impl<Param> DualNum<Param> {
//     fn cos(&self) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         if out.values.is_empty() {
//             return out;
//         }
//         let cos = self.values[0].cos();
//         out.values[0] = cos;
//         if out.size() == 1 {
//             return out;
//         }
//         let sin = self.values[0].sin();
//         let neg_in_deriv = -self.values[1];
//         let deriv = sin * neg_in_deriv;
//         out.values[1] = deriv;
//         if out.size() == 2 {
//             return out;
//         }
//         let int = cos * neg_in_deriv;
//         out.values[2] = int * self.values[1] - sin * self.values[2];
//         if out.size() == 3 {
//             return out;
//         }
//         out.values[3] = deriv * neg_in_deriv * self.values[1] +
//             3.0 * int * self.values[2] -
//             sin * self.values[3];
//         out
//     }
// }
//
// impl<Param, NewParam> DualNum<Param> {
//     /**
//      * Reparameterizes (x, dx/du, d^2x/du^2, ..., d^(n - 1)x/du^(n - 1)) into
//      * (x, dx/dt, d^2x/dt^2, ..., d^(n - 1)x/dt^(n - 1)) using oldParam and
//      * the chain rule.
//      *
//      * @param oldParam (u, du/dt, d^2u/dt^2, ..., d^(n - 1)u/dt^(n - 1))
//      */
//     fn reparam<NewParam>(&self, oldParam: &DualNum<NewParam>) -> DualNum<NewParam> {
//         let size = min(self.size(), oldParam.size());
//         let mut out = DualNum::new(vec![0.0; size]);
//         if out.values.is_empty() {
//             return out;
//         }
//         out.values[0] = self.values[0];
//         if out.size() == 1 {
//             return out;
//         }
//         out.values[1] = self.values[1] * oldParam.get(1);
//         if out.size() == 2 {
//             return out;
//         }
//         let old_deriv2 = oldParam.get(1) * oldParam.get(1);
//         out.values[2] = old_deriv2 * self.values[2] + oldParam.get(2) * self.values[1];
//         if out.size() == 3 {
//             return out;
//         }
//         out.values[3] = self.values[1] * oldParam.get(3) +
//             (3.0 * self.values[2] * oldParam.get(2) + self.values[3] * old_deriv2) * oldParam.get(1);
//         out
//     }
// }
//
// impl<Param> std::ops::Add<f64> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn add(self, c: f64) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         for i in 0..out.size() {
//             out.values[i] = if i == 0 { self.values[0] + c } else { self.values[i] };
//         }
//         out
//     }
// }
//
// impl<Param> std::ops::Sub<f64> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn sub(self, c: f64) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         for i in 0..out.size() {
//             out.values[i] = if i == 0 { self.values[0] - c } else { self.values[i] };
//         }
//         out
//     }
// }
//
// impl<Param> std::ops::Mul<f64> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn mul(self, c: f64) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         for i in 0..out.size() {
//             out.values[i] = self.values[i] * c;
//         }
//         out
//     }
// }
//
// impl<Param> std::ops::Div<f64> for &DualNum<Param> {
//     type Output = DualNum<Param>;
//
//     fn div(self, c: f64) -> DualNum<Param> {
//         let mut out = DualNum::new(vec![0.0; self.size()]);
//         for i in 0..out.size() {
//             out.values[i] = self.values[i] / c;
//         }
//         out
//     }
// }
//
// struct Vector2d {
//     x: f64,
//     y: f64,
// }
//
// struct Vector2dDual<'a, Param> {
//     x: DualNum<Param>,
//     y: DualNum<Param>,
// }
//
// impl<'a, Param> std::ops::Mul<f64> for &'a DualNum<Param> {
//     type Output = Vector2dDual<'a, Param>;
//
//     fn mul(self, c: f64) -> Vector2dDual<'a, Param> {
//         Vector2dDual {
//             x: self * c,
//             y: self * c,
//         }
//     }
// }
