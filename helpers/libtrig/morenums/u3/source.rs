use crate::morenums::bit;

/// The 3-bit unsigned integer type.
#[repr(C)]
#[derive(Clone, Copy, Hash)]
#[allow(non_camel_case_types)]
pub struct u3(pub(crate) bit, pub(crate) bit, pub(crate) bit);

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
    /// Create a new `u3` from three `bit`s.
    #[inline]
    #[must_use]
    pub const fn new(ones: bool, twos: bool, fours: bool) -> Self {
        Self(ones, twos, fours)
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
    #[allow(unsafe_code)]
    pub const unsafe fn fromu8(u: u8) -> Self {
        match u {
            0 => Self::ZERO,
            1 => Self::ONE,
            2 => Self::TWO,
            3 => Self::THREE,
            4 => Self::FOUR,
            5 => Self::FIVE,
            6 => Self::SIX,
            7 => Self::SEVEN,
            _ => core::hint::unreachable_unchecked(),
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
    /// Returns `true` if self is `0`.
    #[must_use]
    #[inline(always)]
    pub const fn zero(self) -> bool {
        !self.0 && !self.1 && !self.2
    }
    /// Returns `true` if self is `1`.
    #[must_use]
    #[inline(always)]
    pub const fn one(self) -> bool {
        self.0 && !self.1 && !self.2
    }
    /// Returns `true` if self is `2`.
    #[must_use]
    #[inline(always)]
    pub const fn two(self) -> bool {
        !self.0 && self.1 && !self.2
    }
    /// Returns `true` if self is `3`.
    #[must_use]
    #[inline(always)]
    pub const fn three(self) -> bool {
        self.0 && self.1 && !self.2
    }
    /// Returns `true` if self is `4`.
    #[must_use]
    #[inline(always)]
    pub const fn four(self) -> bool {
        !self.0 && !self.1 && self.2
    }
    /// Returns `true` if self is `5`.
    #[must_use]
    #[inline(always)]
    pub const fn five(self) -> bool {
        self.0 && !self.1 && self.2
    }
    /// Returns `true` if self is `6`.
    #[must_use]
    #[inline(always)]
    pub const fn six(self) -> bool {
        !self.0 && self.1 && self.2
    }
    /// Returns `true` if self is `7`.
    #[must_use]
    #[inline(always)]
    pub const fn seven(self) -> bool {
        self.0 && self.1 && self.2
    }
    /// Inverts all bits in a `u3`.
    #[must_use]
    #[inline(always)]
    pub const fn bitnot(self) -> Self {
        Self(!self.0, !self.1, !self.2)
    }
    /// Adds two `u3`s.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it may overflow,
    /// therefore causing undefined behavior.
    /// The developer must ensure that the result is in the range 0..=7.
    #[must_use]
    #[inline(always)]
    #[allow(non_snake_case, unsafe_code)]
    pub const unsafe fn unchecked_add(self, rhs: Self) -> Self {
        #[cfg(feature = "unstable")]
        {
            Self::fromu8(self.tou8().unchecked_add(rhs.tou8()))
        }
        #[cfg(not(feature = "unstable"))]
        {
            // If either is zero, the result is the other
            if rhs.zero() {
                return self;
            }
            if self.zero() {
                return rhs;
            }

            let ones = self.0 ^ rhs.0;
            let carry_ones = self.0 & rhs.0;
            let twos = self.1 ^ rhs.1 ^ carry_ones;
            let carry_twos = (self.1 & rhs.1) | (self.1 & carry_ones) | (rhs.1 & carry_ones);
            let fours = self.2 ^ rhs.2 ^ carry_twos;
            Self::new(ones, twos, fours)
        }
    }
    /// Subtracts two `u3`s.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it may overflow,
    /// therefore causing undefined behavior.
    /// The developer must ensure that the result is in the range 0..=7.
    #[must_use]
    #[inline(always)]
    #[allow(non_snake_case, unsafe_code)]
    pub const unsafe fn unchecked_sub(self, rhs: Self) -> Self {
        #[cfg(feature = "unstable")]
        {
            Self::fromu8(self.tou8().unchecked_sub(rhs.tou8()))
        }
        #[cfg(not(feature = "unstable"))]
        {
            // If we are subtracting zero, the result is this
            if rhs.zero() {
                return self;
            }
            // If we are subtracting from zero, the result is undefined
            if self.zero() {
                core::hint::unreachable_unchecked();
            }

            self.unchecked_add(rhs.bitnot()).unchecked_add(Self::ONE)
        }
    }
    /// Performs an unchecked multiplication of two `u3`s.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it may overflow,
    /// therefore causing undefined behavior.
    /// The developer must ensure that the result is in the range 0..=7.
    #[must_use]
    #[inline(always)]
    #[allow(unsafe_code)]
    pub const unsafe fn unchecked_mul(self, rhs: Self) -> Self {
        // If either is zero, the result is zero
        if self.zero() || rhs.zero() {
            return Self::ZERO;
        }

        // If either is one, the result is the other
        if self.one() {
            return rhs;
        }
        if rhs.one() {
            return self;
        }

        // 2 * 2 = 4
        if self.two() && rhs.two() {
            return Self::FOUR;
        }

        // 2 * 3 = 6 OR 3 * 2 = 6
        if (self.two() && rhs.three()) || (self.three() && rhs.two()) {
            return Self::SIX;
        }

        // Otherwise, the result is not allowed
        core::hint::unreachable_unchecked();
    }
    /// Performs an unchecked division of two `u3`s.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it may overflow,
    /// therefore causing undefined behavior.
    /// The developer must ensure that the result is in the range 0..=7.
    #[must_use]
    #[inline(always)]
    #[allow(unsafe_code)]
    pub const unsafe fn unchecked_div(self, rhs: Self) -> Self {
        // if this is zero, the result is zero
        if self.zero() {
            return Self::ZERO;
        }

        // if rhs is zero, the result is undefined
        if rhs.zero() {
            core::hint::unreachable_unchecked();
        }

        // if rhs is one, the result is this
        if rhs.one() {
            return self;
        }

        // If they are equal, the result is one
        if self.0 == rhs.0 && self.1 == rhs.1 && self.2 == rhs.2 {
            return Self::ONE;
        }

        // 4 / 2 = 2
        if self.four() && rhs.two() {
            return Self::TWO;
        }

        // 6 / 2 = 3 OR 6 / 3 = 2
        if self.six() && rhs.two() {
            return Self::THREE;
        }
        if self.six() && rhs.three() {
            return Self::TWO;
        }

        // Otherwise, the result is not allowed
        core::hint::unreachable_unchecked();
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
        Self::new(self.0 & rhs.0, self.1 & rhs.1, self.2 & rhs.2)
    }
}

impl core::ops::BitOr for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    fn bitor(self, rhs: Self) -> Self::Output {
        Self::new(self.0 | rhs.0, self.1 | rhs.1, self.2 | rhs.2)
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
        self.bitnot()
    }
}

impl core::ops::Add for u3 {
    type Output = Self;
    #[must_use]
    #[inline(always)]
    #[cfg(debug_assertions)]
    fn add(self, rhs: Self) -> Self::Output {
        let new = self.tou8() + rhs.tou8();
        assert!(new <= 7, "u3 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[allow(unsafe_code)]
    #[cfg(not(debug_assertions))]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self::unchecked_add(self, rhs) }
    }
}

impl core::ops::Sub for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    #[cfg(debug_assertions)]
    fn sub(self, rhs: Self) -> Self::Output {
        let new = self.tou8() - rhs.tou8();
        assert!(new <= 7, "u3 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[allow(unsafe_code)]
    #[cfg(not(debug_assertions))]
    fn sub(self, rhs: Self) -> Self::Output {
        unsafe { Self::unchecked_sub(self, rhs) }
    }
}

impl core::ops::Mul for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    #[cfg(debug_assertions)]
    fn mul(self, rhs: Self) -> Self::Output {
        let new = self.tou8() * rhs.tou8();
        assert!(new <= 7, "u3 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[allow(unsafe_code)]
    #[cfg(not(debug_assertions))]
    fn mul(self, rhs: Self) -> Self::Output {
        unsafe { self.unchecked_mul(rhs) }
    }
}

impl core::ops::Div for u3 {
    type Output = Self;
    #[inline]
    #[must_use]
    #[cfg(debug_assertions)]
    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(rhs, Self::ZERO, "u3 division by zero");
        let new = self.tou8() / rhs.tou8();
        assert!(new <= 7, "u3 out of range: {}", new);
        new.into()
    }
    #[must_use]
    #[inline(always)]
    #[allow(unsafe_code)]
    #[cfg(not(debug_assertions))]
    fn div(self, rhs: Self) -> Self::Output {
        unsafe { self.unchecked_div(rhs) }
    }
}
