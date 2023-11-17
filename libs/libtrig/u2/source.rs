/// The 2-bit unsigned integer type.
#[repr(C)]
#[derive(Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub struct u2(bool, bool);

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
        let carry: bool = self.0 && rhs.0;
        let ones: bool = self.0 ^ rhs.0 ^ carry;
        let twos: bool = self.1 ^ rhs.1;
        Self::new(ones, twos)
    }
}

impl From<u2> for u8 {
    /// Convert a `u2` to a `u8`.
    #[must_use]
    #[inline(always)]
    fn from(u: u2) -> Self {
        if u.0 && u.1 {
            3
        } else if u.0 {
            1
        } else if u.1 {
            2
        } else {
            0
        }
    }
}

impl From<u8> for u2 {
    /// Convert a `u8` to a `u2`.
    ///
    /// # Safety
    ///
    /// In debug mode, this will panic if the `u8` is out of range. In release
    /// mode, this uses `core::hint::unreachable_unchecked` to enable optimizations.
    #[must_use]
    #[inline(always)]
    fn from(u: u8) -> Self {
        #[cfg(debug_assertions)]
        debug_assert!(u <= 3, "u8 out of range: {}", u);
        match u {
            0 => Self::ZERO,
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            #[cfg(debug_assertions)]
            _ => panic!("u8 out of range: {}", u),
            #[cfg(not(debug_assertions))]
            _ => unsafe { core::hint::unreachable_unchecked() },
        }
    }
}
