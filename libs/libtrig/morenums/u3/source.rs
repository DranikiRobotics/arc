/// The 3-bit unsigned integer type.
#[repr(C)]
#[derive(Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub struct u3(pub(crate) bool, pub(crate) bool, pub(crate) bool);

impl u3 {
    /// Maximum value of `u3`. (7)
    pub const MAX: Self = Self::new(true, true, true);
    /// Minimum value of `u3`. (0)
    pub const MIN: Self = Self::new(false, false, false);
    /// Zero value of `u3`. (0)
    pub const ZERO: Self = Self::MIN;
    /// One value of `u3`. (1)
    pub const ONE: Self = Self::new(true, false, false);
    /// Two value of `u3`. (2)
    pub const TWO: Self = Self::new(false, true, false);
    /// Three value of `u3`. (3)
    pub const THREE: Self = Self::new(true, true, false);
    /// Four value of `u3`. (4)
    pub const FOUR: Self = Self::new(false, false, true);
    /// Five value of `u3`. (5)
    pub const FIVE: Self = Self::new(true, false, true);
    /// Six value of `u3`. (6)
    pub const SIX: Self = Self::new(false, true, true);
    /// Seven value of `u3`. (7)
    pub const SEVEN: Self = Self::MAX;
    /// Create a new `u3` from three `bool`s.
    #[inline]
    #[must_use]
    pub const fn new(ones: bool, twos: bool, fours: bool) -> Self {
        Self(ones, twos, fours)
    }
    /// Adds two `u3`s.
    #[inline]
    #[must_use]
    #[allow(non_snake_case)]
    pub const fn bitwiseXORadd(self, rhs: Self) -> Self {
        let mut result = self;
        if rhs.0 {
            result.0 ^= true;
            if !result.0 {
                result.1 ^= true;
                if !result.1 {
                    result.2 ^= true;
                }
            }
        }
        if rhs.1 {
            result.1 ^= true;
            if !result.1 {
                result.2 ^= true;
            }
        }
        if rhs.2 {
            result.2 ^= true;
        }
        result
    }
    /// Convert a `u3` to a `u8`.
    #[must_use]
    #[inline(always)]
    pub const fn tou8(self) -> u8 {
        if self.0 && self.1 && self.2 {
            7
        } else if !self.0 && self.1 && self.2 {
            6
        } else if self.0 && !self.1 && self.2 {
            5
        } else if !self.0 && !self.1 && self.2 {
            4
        } else if self.0 && self.1 && !self.2 {
            3
        } else if !self.0 && self.1 && !self.2 {
            2
        } else if self.0 && !self.1 && !self.2 {
            1
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
            4 => Self::FOUR,
            5 => Self::FIVE,
            6 => Self::SIX,
            7 => Self::SEVEN,
            #[allow(unsafe_code)]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
    /// Convert a `u3` to a `u2`.
    #[must_use]
    #[inline(always)]
    pub const fn tou2(self) -> crate::u2 {
        crate::u2::new(self.0, self.1)
    }
    /// Convert a `u2` to a `u3`.
    #[must_use]
    #[inline(always)]
    pub const fn fromu2(u: crate::u2) -> Self {
        Self::new(u.0, u.1, false)
    }
}

impl core::fmt::Debug for u3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_set()
            .entry(&self.0)
            .entry(&self.1)
            .entry(&self.2)
            .finish()
    }
}

impl core::cmp::PartialEq for u3 {
    #[inline]
    #[must_use]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl core::cmp::PartialOrd for u3 {
    #[inline]
    #[must_use]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if self.0 == other.0 && self.1 == other.1 && self.2 == other.2 {
            Some(core::cmp::Ordering::Equal)
        } else if self.2 && !other.2 {
            Some(core::cmp::Ordering::Greater)
        } else if !self.2 && other.2 {
            Some(core::cmp::Ordering::Less)
        } else if self.1 && !other.1 {
            Some(core::cmp::Ordering::Greater)
        } else if !self.1 && other.1 {
            Some(core::cmp::Ordering::Less)
        } else if self.0 && !other.0 {
            Some(core::cmp::Ordering::Greater)
        } else if !self.0 && other.0 {
            Some(core::cmp::Ordering::Less)
        } else {
            None
        }
    }
}

impl core::cmp::Ord for u3 {
    /// Compare two `u3`s.
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

impl core::ops::BitAnd for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitand(self, rhs: Self) -> Self::Output {
        Self::new(self.0 && rhs.0, self.1 && rhs.1, self.2 && rhs.2)
    }
}

impl core::ops::BitOr for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 || rhs.0, self.1 || rhs.1, self.2 || rhs.2)
    }
}

impl core::ops::BitXor for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 ^ rhs.0, self.1 ^ rhs.1, self.2 ^ rhs.2)
    }
}

impl core::ops::Not for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn not(self) -> Self::Output {
        Self::new(!self.0, !self.1, !self.2)
    }
}

impl core::ops::Add for u3 {
    type Output = Self;
    #[must_use]
    #[inline(always)]
    #[cfg(debug_assertions)]
    fn add(self, rhs: Self) -> Self::Output {
        let new = u8::from(self) + u8::from(rhs);
        debug_assert!(new <= 7, "u3 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn add(self, rhs: Self) -> Self::Output {
        Self::bitwiseXORadd(self, rhs)
    }
}

impl core::ops::AddAssign for u3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl core::ops::Sub for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    #[cfg(debug_assertions)]
    fn sub(self, rhs: Self) -> Self::Output {
        let new = u8::from(self) - u8::from(rhs);
        debug_assert!(new <= 7, "u3 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[cfg(not(debug_assertions))]
    fn sub(self, rhs: Self) -> Self::Output {
        Self::bitwiseXORadd(self, rhs)
    }
}
