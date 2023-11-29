/// The 2-bit unsigned integer type.
#[repr(C)]
#[derive(Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub struct u2(pub(crate) bool, pub(crate) bool);

impl u2 {
    /// Maximum value of `u2`. (3)
    pub const MAX: Self = Self::new(true, true);
    /// Minimum value of `u2`. (0)
    pub const MIN: Self = Self::new(false, false);
    /// Zero value of `u2`. (0)
    pub const ZERO: Self = Self::MIN;
    /// One value of `u2`. (1)
    pub const ONE: Self = Self::new(true, false);
    /// Two value of `u2`. (2)
    pub const TWO: Self = Self::new(false, true);
    /// Three value of `u2`. (3)
    pub const THREE: Self = Self::MAX;
    /// Create a new `u2` from two `bool`s.
    #[inline]
    #[must_use]
    pub const fn new(ones: bool, twos: bool) -> Self {
        Self(ones, twos)
    }
    /// Adds two `u2`s.
    #[inline]
    #[must_use]
    #[allow(non_snake_case)]
    pub const fn bitwiseXORadd(self, rhs: Self) -> Self {
        let mut result = self;
        if rhs.0 {
            result.0 ^= true;
            if !result.0 {
                result.1 ^= true;
            }
        }
        if rhs.1 {
            result.1 ^= true;
        }
        result
    }
    /// Convert a `u2` to a `u8`.
    #[must_use]
    #[inline(always)]
    pub const fn tou8(self) -> u8 {
        if self.0 && self.1 {
            3
        } else if self.0 {
            1
        } else if self.1 {
            2
        } else {
            0
        }
    }
    /// Convert a `u8` to a `u3`.
    ///
    /// # Safety
    ///
    /// This uses `core::hint::unreachable_unchecked` to enable optimizations, so
    /// it is unsafe to use in debug mode. Instead, use `u3::fromu8`.
    #[must_use]
    #[inline(always)]
    #[cfg_attr(all(test, assert_no_panic), no_panic::no_panic)]
    pub const fn fromu8(u: u8) -> Self {
        match u {
            0 => Self::ZERO,
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            #[allow(unsafe_code)]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    /// Convert a `u2` to a `u3`.
    #[must_use]
    #[inline(always)]
    pub const fn tou3(self) -> crate::u3 {
        crate::u3::new(self.0, self.1, false)
    }
    /// Convert a `u3` to a `u2`.
    #[must_use]
    #[inline(always)]
    pub const fn fromu3(u: crate::u3) -> Self {
        Self::new(u.0, u.1)
    }
}

impl core::fmt::Debug for u2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set().entry(&self.0).entry(&self.1).finish()
    }
}

impl core::cmp::PartialEq for u2 {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl core::cmp::PartialOrd for u2 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.0 == other.0 && self.0 == other.0 {
            return Some(core::cmp::Ordering::Equal);
        }
        if self.0 && !other.0 {
            return Some(core::cmp::Ordering::Greater);
        }
        if !self.0 && other.0 {
            return Some(core::cmp::Ordering::Less);
        }
        if self.1 && !other.1 {
            return Some(core::cmp::Ordering::Greater);
        }
        if !self.1 && other.1 {
            return Some(core::cmp::Ordering::Less);
        }
        None
    }
}

impl core::cmp::Ord for u2 {
    /// Compare two `u2`s.
    ///
    /// # Safety
    ///
    /// This is safe because we are comparing two `u2`s, which are guaranteed to
    /// be two valid `bool`s.
    #[inline]
    #[must_use]
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        #[allow(unsafe_code)]
        #[cfg(not(debug_assertions))]
        unsafe {
            self.partial_cmp(other).unwrap_unchecked()
        }
        #[cfg(debug_assertions)]
        self.partial_cmp(other).unwrap()
    }
}

impl core::ops::BitAnd for u2 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.0 && rhs.0, self.1 && rhs.1)
    }
}

impl core::ops::BitOr for u2 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 || rhs.0, self.1 || rhs.1)
    }
}

impl core::ops::BitXor for u2 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}

impl core::ops::Not for u2 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn not(self) -> Self::Output {
        Self::new(!self.0, !self.1)
    }
}

impl core::ops::Add for u2 {
    type Output = Self;
    #[cfg(debug_assertions)]
    fn add(self, rhs: Self) -> Self::Output {
        let new = u8::from(self) + u8::from(rhs);
        debug_assert!(new <= 3, "u2 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn add(self, rhs: Self) -> Self::Output {
        let carry: bool = self.0 && rhs.0;
        let ones: bool = self.0 ^ rhs.0;
        let twos: bool = self.1 ^ rhs.1 ^ carry;
        Self::new(ones, twos)
    }
}

impl core::ops::AddAssign for u2 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::Sub for u2 {
    type Output = Self;
    #[inline]
    #[must_use]
    #[cfg(debug_assertions)]
    fn sub(self, rhs: Self) -> Self::Output {
        let new = u8::from(self) - u8::from(rhs);
        debug_assert!(new <= 3, "u2 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::bitwiseXORadd(self, rhs)
    }
}
