use super::source::*;

impl core::fmt::Debug for u2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", u8::from(*self))
    }
}

impl core::fmt::Display for u2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", u8::from(*self))
    }
}

impl core::cmp::Eq for u2 {}

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
