use core::marker::PhantomData;
use libtrig::prelude::*;
use libtrig::*;

type Array<T> = alloc::vec::Vec<T>;

/// [Dual number](https://en.wikipedia.org/wiki/Dual_number)
/// to implement forward autodifferentiation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct DualNum<T>(Array<Float64>, PhantomData<T>);

impl<T> DualNum<T> {
    #[inline(always)]
    #[must_use = "this returns the dual number (new)"]
    pub(crate) const fn constructor(array: Array<Float64>) -> Self {
        Self(array, PhantomData)
    }

    /// Makes dual number (c, 0, ..., 0) with size [n] representing a constant function (c).
    pub fn constant(c: Float64, n: usize) -> Self {
        let mut array = Array::with_capacity(n);
        for i in 0..n {
            array.push(if i == 0 { c } else { 0.0 });
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }

    /// Makes dual number (x0, 1, 0, ..., 0) with size [n] representing a variable function (x) at (x = x0).
    pub fn variable(x0: Float64, n: usize) -> Self {
        let mut array = Array::with_capacity(n);
        for i in 0..n {
            array.push(if i == 0 {
                x0
            } else if i == 1 {
                1.0
            } else {
                0.0
            });
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }

    /// Constructs a dual number (x, d[0], d[1], ..., d[n-1]) with size [n] representing a constant function (x).
    pub fn cons(x: Float64, d: Self) -> Self {
        let mut array = Array::with_capacity(d.size() + 1);
        for i in 0..array.capacity() {
            array.push(if i == 0 { x } else { d.values()[i - 1] });
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }

    /// Returns the size of the dual number.
    #[inline(always)]
    pub fn size(&self) -> usize {
        self.0.len() as usize
    }

    /// Returns the values of the dual number.
    #[inline(always)]
    pub fn values(&self) -> &[Float64] {
        &self.0
    }

    /// Returns the value of the dual number.
    #[inline(always)]
    #[must_use = "this returns the first component of the dual number"]
    pub fn value(&self) -> Float64 {
        self.0[0]
    }

    /// Drops the first [n] components of the dual number.
    ///
    /// # Panics
    ///
    /// Panics if [n] is greater than the size of the dual number.
    #[must_use = "this returns a new dual number"]
    pub fn drop(&self, n: usize) -> Self {
        let mut array = Array::with_capacity(self.size() - n);
        for i in 0..array.capacity() {
            array[i] = self.0[i + n];
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }

    /// Calculates the reciprocal of the dual number.
    #[must_use = "this returns the reciprocal of the dual number (new)"]
    pub fn recip(&self) -> Self {
        let mut array = Array::with_capacity(self.size());
        if array.capacity() == 0 {
            return Self::constructor(array);
        }

        let recip = 1.0 / self.0[0];
        array[0] = recip;
        if array.capacity() == 1 {
            return Self::constructor(array);
        }

        let neg_recip = -recip;
        let neg_recip2 = recip * neg_recip;
        let deriv = neg_recip2 * self.0[1];
        array[1] = deriv;
        if array.capacity() == 2 {
            return Self::constructor(array);
        }

        let int1 = 2.0 * neg_recip * deriv;
        let deriv2 = int1 * self.0[1] + neg_recip2 * self.0[2];
        array[2] = deriv2;
        if array.capacity() == 3 {
            return Self::constructor(array);
        }

        let int2 = int1 * self.0[2];
        array[3] = 
            int2 + neg_recip2 * self.0[3] +
            int2 - 2.0 * (deriv * deriv + recip * deriv2) * self.0[1];
        Self::constructor(array)
    }

    /// Reparameterizes (x, dx/du, d^2x/du^2, ..., d^(n - 1) x/du^(n - 1)) into
    /// (x, dx/dt, d^2x/dt^2, ..., d^(n - 1) x/dt^(n - 1)) using [old_param] and
    /// the chain rule.
    pub fn reparam<NewT>(&self, old_param: DualNum<NewT>) -> DualNum<NewT> {
        let mut array = Array::with_capacity(self.size().min(old_param.size()));
        if array.capacity() == 0 {
            return DualNum::<NewT>::constructor(array);
        }

        array[0] = self[0];
        if array.capacity() == 1 {
            return DualNum::<NewT>::constructor(array);
        }

        array[1] = self[1] * old_param[1];
        if array.capacity() == 2 {
            return DualNum::<NewT>::constructor(array);
        }

        let old_deriv2 = old_param[1] * old_param[1];
        array[2] = old_deriv2 * self[2] + old_param[2] * self[1];
        if array.capacity() == 3 {
            return DualNum::<NewT>::constructor(array);
        }

        array[3] = self[1] * old_param[3] +
            (3.0 * self[2] * old_param[2] + self[3] * old_deriv2) * old_param[1];
        DualNum::<NewT>::constructor(array)
    }
}

impl<T> Sqrt for DualNum<T> {
    /// Calculates the square root of the dual number.
    #[must_use = "this returns the square root of the dual number (new)"]
    fn sqrt(&self) -> Self {
        let mut array = Array::with_capacity(self.size());
        if array.capacity() == 0 {
            return Self::constructor(array);
        }

        let sqrt = l2math::sqrt(self.0[0]);
        array[0] = sqrt;
        if array.capacity() == 1 {
            return Self::constructor(array);
        }

        let recip = 1.0 / (2.0 * sqrt);
        let deriv = recip * self.0[1];
        array[1] = deriv;
        if array.capacity() == 2 {
            return Self::constructor(array);
        }

        let neg_recip = -2.0 * recip;
        let neg_recip2 = recip * neg_recip;
        let int1 = neg_recip2 * deriv;
        let second_deriv = int1 * self.0[1] + recip * self.0[2];
        array[2] = second_deriv;
        if array.capacity() == 3 {
            return Self::constructor(array);
        }

        let int2 = 2.0 * int1;
        array[3] = recip * self.0[3] + int2 * self.0[2] +
            (deriv * neg_recip * int2 + neg_recip2 * second_deriv) * self.0[1];
        Self::constructor(array)
    }
}

impl<T> Sin for DualNum<T> {
    /// Calculates the sine of the dual number.
    #[must_use = "this returns the sine of the dual number (new)"]
    fn sin(&self) -> Self {
        let mut array = Array::with_capacity(self.size());
        if array.capacity() == 0 {
            return Self::constructor(array);
        }

        let sin = l2math::sin(self.0[0]);
        array[0] = sin;
        if array.capacity() == 1 {
            return Self::constructor(array);
        }

        let cos = l2math::cos(self.0[0]);
        let deriv = cos * self.0[1];
        array[1] = deriv;
        if array.capacity() == 2 {
            return Self::constructor(array);
        }

        let deriv2 = self.0[1] * self.0[1];
        array[2] = cos * self.0[2] - sin * deriv2;
        if array.capacity() == 3 {
            return Self::constructor(array);
        }

        array[3] = cos * self.0[3] -
            3.0 * sin * self.0[1] * self.0[2] -
            deriv * deriv2;
        Self::constructor(array)
    }
}

impl<T> Cos for DualNum<T> {
    /// Calculates the cosine of the dual number.
    #[must_use = "this returns the cosine of the dual number (new)"]
    fn cos(&self) -> Self {
        let mut array = Array::with_capacity(self.size());
        if array.capacity() == 0 {
            return Self::constructor(array);
        }

        let cos = l2math::cos(self.0[0]);
        array[0] = cos;
        if array.capacity() == 1 {
            return Self::constructor(array);
        }

        let sin = l2math::sin(self.0[0]);
        let neg_in_eriv = -self.0[1];
        let deriv = sin * neg_in_eriv;
        array[1] = deriv;
        if array.capacity() == 2 {
            return Self::constructor(array);
        }

        let int = cos * neg_in_eriv;
        array[2] = int * self.0[1] - sin * self.0[2];
        if array.capacity() == 3 {
            return Self::constructor(array);
        }

        array[3] = deriv * neg_in_eriv * self.0[1] +
            3.0 * int * self.0[2] -
            sin * self.0[3];
        Self::constructor(array)
    }
}

impl<T> core::ops::Add for DualNum<T> {
    type Output = Self;
    #[must_use = "this returns the sum of the dual numbers (new)"]
    fn add(self, rhs: Self) -> Self::Output {
        let mut array = Array::with_capacity(self.size().min(rhs.size()));
        for i in 0..array.capacity() {
            array[i] = self.0[i] + rhs.0[i];
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::Add<Float64> for DualNum<T> {
    type Output = Self;
    #[must_use = "this returns the sum of the dual numbers (new)"]
    fn add(self, rhs: Float64) -> Self::Output {
        let mut array = Array::with_capacity(self.size());
        for i in 0..array.capacity() {
            array[i] = self.0[i] + rhs;
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::AddAssign for DualNum<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..self.size().min(rhs.size()) {
            self.0[i] += rhs.0[i];
        }
    }
}

impl<T> core::ops::Sub for DualNum<T> {
    type Output = Self;
    #[must_use = "this returns the difference of the dual numbers (new)"]
    fn sub(self, rhs: Self) -> Self::Output {
        let mut array = Array::with_capacity(self.size().min(rhs.size()));
        for i in 0..array.capacity() {
            array[i] = self.0[i] - rhs.0[i];
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::Sub<Float64> for DualNum<T> {
    type Output = Self;
    #[must_use = "this returns the difference of the dual numbers (new)"]
    fn sub(self, rhs: Float64) -> Self::Output {
        let mut array = Array::with_capacity(self.size());
        for i in 0..array.capacity() {
            array[i] = self.0[i] - rhs;
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::SubAssign for DualNum<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..self.size().min(rhs.size()) {
            self.0[i] -= rhs.0[i];
        }
    }
}

impl<T> core::ops::Mul for DualNum<T> {
    type Output = Self;
    #[rustfmt::skip]
    #[must_use = "this returns the product of the dual numbers (new)"]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut array = Array::with_capacity(self.size().min(rhs.size()));
        if array.capacity() == 0 {
            return Self::constructor(array);
        }

        array[0] = self[0] * rhs[0];
        if array.capacity() == 1 {
            return Self::constructor(array);
        }

        array[1] = self[0] * rhs[1] + self[1] * rhs[0];
        if array.capacity() == 2 {
            return Self::constructor(array);
        }

        array[2] = self[0] * rhs[2] + self[2] * rhs[0] + 2.0 * self[1] * rhs[1];
        if array.capacity() == 3 {
            return Self::constructor(array);
        }

        array[3] = self[0] * rhs[3] + self[3] * rhs[0] +
            3.0 * (self[2] * rhs[1] + self[1] * rhs[2]);

        return Self::constructor(array);
    }
}

impl<T> core::ops::Mul<Float64> for DualNum<T> {
    type Output = Self;
    #[must_use = "this returns the product of the dual numbers (new)"]
    fn mul(self, rhs: Float64) -> Self::Output {
        let mut array = Array::with_capacity(self.size());
        for i in 0..array.capacity() {
            array[i] = self.0[i] * rhs;
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::Neg for DualNum<T> {
    type Output = Self;

    #[must_use = "this returns the negation of the dual number (new)"]
    fn neg(self) -> Self::Output {
        let mut array = Array::with_capacity(self.size());
        for i in 0..array.capacity() {
            array[i] = -self.0[i];
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::Div for DualNum<T> {
    type Output = Self;
    #[inline]
    #[must_use = "this returns the quotient of the dual numbers (new)"]
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
    }
}

impl<T> core::ops::Div<Float64> for DualNum<T> {
    type Output = Self;
    #[inline]
    #[must_use = "this returns the quotient of the dual numbers (new)"]
    fn div(self, rhs: Float64) -> Self::Output {
        let mut array = Array::with_capacity(self.size());
        for i in 0..array.capacity() {
            array[i] = self.0[i] / rhs;
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }
}

impl<T> core::ops::Index<usize> for DualNum<T> {
    type Output = Float64;
    #[inline(always)]
    #[must_use = "this returns the first component of the dual number"]
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> core::ops::Index<usize> for &DualNum<T> {
    type Output = Float64;
    #[inline(always)]
    #[must_use = "this returns the first component of the dual number"]
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> core::ops::Index<usize> for &mut DualNum<T> {
    type Output = Float64;
    #[inline(always)]
    #[must_use = "this returns the first component of the dual number"]
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl<T> core::ops::IndexMut<usize> for DualNum<T> {
    #[inline(always)]
    #[must_use = "this returns the first component of the dual number"]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl<T> core::ops::IndexMut<usize> for &mut DualNum<T> {
    #[inline(always)]
    #[must_use = "this returns the first component of the dual number"]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}
