#[allow(unused_imports)]
use libtrig::prelude::*;
use libtrig::*;

use crate::DualNum;

/// Dual version of a 2D vector.
pub struct Vec2DDual<T> {
    /// The x component of the vector.
    pub x: DualNum<T>,
    /// The y component of the vector.
    pub y: DualNum<T>,
}

impl<T> Vec2DDual<T> {
    /// Construct a new Vec2DDual.
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    pub const fn new(x: DualNum<T>, y: DualNum<T>) -> Self {
        Self { x, y }
    }
    /// Construct a new Vec2DDual from a Vec2D.
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    pub fn constant(v: Vec2D, n: usize) -> Self {
        Self::new(DualNum::constant(v.x(), n), DualNum::constant(v.y(), n))
    }
}

impl<T> core::ops::Add<Vec2D> for Vec2DDual<T> {
    type Output = Self;
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    fn add(self, rhs: Vec2D) -> Self::Output {
        Self::new(self.x + rhs.x(), self.y + rhs.y())
    }
}

impl<T> core::ops::Add<Vec2DDual<T>> for Vec2DDual<T> {
    type Output = Self;
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    fn add(self, rhs: Vec2DDual<T>) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<T> core::ops::Sub<Vec2D> for Vec2DDual<T> {
    type Output = Self;
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    fn sub(self, rhs: Vec2D) -> Self::Output {
        Self::new(self.x - rhs.x(), self.y - rhs.y())
    }
}

impl<T> core::ops::Sub<Vec2DDual<T>> for Vec2DDual<T> {
    type Output = Self;
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    fn sub(self, rhs: Vec2DDual<T>) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<T> core::ops::Neg for Vec2DDual<T> {
    type Output = Self;
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl<T> core::ops::Div<Float64> for Vec2DDual<T> {
    type Output = Self;
    #[inline]
    #[must_use = "This function creates a new Vec2DDual"]
    fn div(self, rhs: Float64) -> Self::Output {
        Self::new(self.x / rhs, self.y / rhs)
    }
}

// impl Dot for 
