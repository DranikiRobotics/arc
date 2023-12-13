use core::ops;

/// A trait for types that can be used as numbers.
pub trait Number<Multiplier: Number = Self>:
    core::fmt::Debug
    + Copy
    + Clone
    + PartialEq
    + ops::Add<Output = Self>
    + ops::AddAssign
    + ops::Sub<Output = Self>
    + ops::SubAssign
    + ops::Mul<Multiplier, Output = Self>
    + ops::MulAssign
    + ops::Div<Multiplier, Output = Self>
    + ops::DivAssign
    + From<crate::u3>
{
}

/// A trait that calculates the dot product of two items.
pub trait Dot<Rhs = Self> {
    /// The output type of the dot product.
    type Output;
    /// Calculate the dot product of two items.
    #[must_use]
    fn dot(self, rhs: Rhs) -> Self::Output;
}
