use super::source::*;

impl core::fmt::Display for u3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

impl core::default::Default for u3 {
    #[must_use]
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl core::cmp::Eq for u3 {}

impl From<u3> for u8 {
    /// Convert a `u2` to a `u8`.
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u3::tou8(u)
    }
}

impl From<u8> for u3 {
    /// Convert a `u8` to a `u3`.
    ///
    /// # Safety
    ///
    /// In debug mode, this will panic if the `u8` is out of range. In release
    /// mode, this uses `core::hint::unreachable_unchecked` to enable optimizations.
    #[must_use]
    #[inline(always)]
    fn from(u: u8) -> Self {
        debug_assert!(u <= 7, "u8 out of range: {}", u);
        // Safety: u is in range 0..=7
        // In debug mode, this is verified by the debug_assert above.
        // In release mode, this uses `core::hint::unreachable_unchecked` to enable optimizations.
        #[allow(unsafe_code)]
        unsafe {
            Self::fromu8(u)
        }
    }
}

impl From<u3> for u16 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as u16
    }
}

impl From<u3> for u32 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as u32
    }
}

impl From<u3> for u64 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as u64
    }
}

impl From<u3> for u128 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as u128
    }
}

impl From<u3> for usize {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as usize
    }
}

impl From<u3> for i8 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as i8
    }
}

impl From<u3> for i16 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as i16
    }
}

impl From<u3> for i32 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as i32
    }
}

impl From<u3> for i64 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as i64
    }
}

impl From<u3> for i128 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as i128
    }
}

impl From<u3> for isize {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as isize
    }
}

impl From<u3> for crate::Float32 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as crate::Float32
    }
}

impl From<u3> for crate::Float64 {
    #[must_use]
    #[inline(always)]
    fn from(u: u3) -> Self {
        u8::from(u) as crate::Float64
    }
}

impl core::ops::AddAssign for u3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::SubAssign for u3 {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl core::ops::MulAssign for u3 {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl core::ops::DivAssign for u3 {
    #[inline]
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl crate::traits::Number for u3 {}
