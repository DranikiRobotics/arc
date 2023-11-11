use core::ops;

/// A trait for types that can be used as numbers.
pub trait Number:
    core::fmt::Debug
    + Copy
    + Clone
    + PartialEq
    + ops::Add<Output = Self>
    + ops::AddAssign
    + ops::Sub<Output = Self>
    + ops::SubAssign
    + ops::Mul<Output = Self>
    + ops::MulAssign
    + ops::Div<Output = Self>
    + ops::DivAssign
{
}

impl Number for crate::Int {}
impl Number for crate::Float64 {}
