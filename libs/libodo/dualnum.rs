use core::marker::PhantomData;
use libtrig::prelude::*;
use libtrig::*;

type Array<T> = alloc::vec::Vec<T>;

/// [Dual number](https://en.wikipedia.org/wiki/Dual_number)
/// to implement forward autodifferentiation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct DualNum<T>(Array<Float64>, PhantomData<T>);

impl<T> DualNum<T> {
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
            array.push(if i == 0 { x0 } else if i == 1 { 1.0 } else { 0.0 });
        }
        debug_assert!(array.len() <= 4);
        Self::constructor(array)
    }

    // @JvmStatic
    // fun <Param> cons(x: Double, d: DualNum<Param>) =
    //     DualNum<Param>(
    //         DoubleArray(d.size() + 1) {
    //             if (it == 0) {
    //                 x
    //             } else {
    //                 d.values[it - 1]
    //             }
    //         }
    //     )

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
        self.0.len()
    }

    pub fn values(&self) -> &[Float64] {
        &self.0
    }

}
