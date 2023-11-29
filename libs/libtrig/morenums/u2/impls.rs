use super::source::*;

impl core::fmt::Display for u2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", u8::from(*self))
    }
}

impl core::default::Default for u2 {
    #[must_use]
    #[inline(always)]
    fn default() -> Self {
        Self::ZERO
    }
}

impl core::cmp::Eq for u2 {}

impl From<u2> for u8 {
    /// Convert a `u2` to a `u8`.
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u2::tou8(u)
    }
}

impl From<u8> for u2 {
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
        Self::fromu8(u)
    }
}

impl From<u2> for u16 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as u16
    }
}

impl From<u2> for u32 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as u32
    }
}

impl From<u2> for u64 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as u64
    }
}

impl From<u2> for u128 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as u128
    }
}

impl From<u2> for usize {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as usize
    }
}

impl From<u2> for i8 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as i8
    }
}

impl From<u2> for i16 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as i16
    }
}

impl From<u2> for i32 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as i32
    }
}

impl From<u2> for i64 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as i64
    }
}

impl From<u2> for i128 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as i128
    }
}

impl From<u2> for isize {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as isize
    }
}

impl From<u2> for crate::Float32 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as crate::Float32
    }
}

impl From<u2> for crate::Float64 {
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        u8::from(u) as crate::Float64
    }
}
