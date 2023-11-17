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
    + From<crate::u2>
{
}

impl Number for u8 {}
impl Number for u16 {}
impl Number for u32 {}
impl Number for u64 {}
impl Number for u128 {}
impl Number for usize {}
impl Number for i8 {}
impl Number for i16 {}
impl Number for i32 {}
impl Number for i64 {}
impl Number for i128 {}
impl Number for isize {}
impl Number for crate::Float32 {}
impl Number for crate::Float64 {}
