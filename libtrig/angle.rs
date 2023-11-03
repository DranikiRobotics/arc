use crate::*;

/// A wrapper around a `float` value that represents an angle.
///
/// It can be created from either radians or degrees, and can be converted to either radians or degrees.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle2D(float, bool);

impl Angle2D {
    /// Creates a new `Angle2D` from the given `value` and `is_radians` flag.
    #[inline]
    #[must_use]
    pub const fn new(value: float, is_radians: bool) -> Self {
        Self(value, is_radians)
    }
    /// Creates a new `Angle2D` from the given `value` in radians.
    #[inline]
    #[must_use]
    pub const fn from_radians(value: float) -> Self {
        Self::new(value, true)
    }
    /// Creates a new `Angle2D` from the given `value` in degrees.
    #[inline]
    #[must_use]
    pub const fn from_degrees(value: float) -> Self {
        Self::new(value, false)
    }
    /// Creates a new `Angle2D` with a value of `0.0` radians.
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0.0, true)
    }
    /// Creates a new `Angle2D` with a value of `0.0` degrees.
    #[inline]
    #[must_use]
    pub const fn zero_deg() -> Self {
        Self::new(0.0, false)
    }
    /// Returns the value of the angle in radians.
    #[inline]
    #[must_use]
    pub fn to_radians(&self) -> float {
        if self.1 {
            self.0
        } else {
            self.0.to_radians()
        }
    }
    /// Returns the value of the angle in degrees.
    #[inline]
    #[must_use]
    pub fn to_degrees(&self) -> float {
        if self.1 {
            self.0.to_degrees()
        } else {
            self.0
        }
    }
}

#[macros::mass_impl(
    $THIS = @ORM Angle2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() + rhs.to_radians())
        } else {
            Angle2D::from_degrees(self.to_degrees() + rhs.to_degrees())
        }
    }
}

#[macros::mass_impl(
    $THIS = @ORM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() + rhs)
        } else {
            Angle2D::from_degrees(self.to_degrees() + rhs)
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() + rhs.to_radians();
        } else {
            self.0 = self.to_degrees() + rhs.to_degrees();
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() + rhs;
        } else {
            self.0 = self.to_degrees() + rhs;
        }
    }
}

#[macros::mass_impl(
    $THIS = @ORM Angle2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() - rhs.to_radians())
        } else {
            Angle2D::from_degrees(self.to_degrees() - rhs.to_degrees())
        }
    }
}

#[macros::mass_impl(
    $THIS = @ORM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() - rhs)
        } else {
            Angle2D::from_degrees(self.to_degrees() - rhs)
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR Angle2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() - rhs.to_radians();
        } else {
            self.0 = self.to_degrees() - rhs.to_degrees();
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() - rhs;
        } else {
            self.0 = self.to_degrees() - rhs;
        }
    }
}

#[macros::mass_impl(
    $THIS = @ORM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::Mul<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn mul(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() * rhs)
        } else {
            Angle2D::from_degrees(self.to_degrees() * rhs)
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::MulAssign<OTHER> for THIS {
    #[inline]
    fn mul_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() * rhs;
        } else {
            self.0 = self.to_degrees() * rhs;
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::Div<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn div(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() / rhs)
        } else {
            Angle2D::from_degrees(self.to_degrees() / rhs)
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR float
)]
impl core::ops::DivAssign<OTHER> for THIS {
    #[inline]
    fn div_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() / rhs;
        } else {
            self.0 = self.to_degrees() / rhs;
        }
    }
}

#[macros::mass_impl($THIS = @ORM Angle2D)]
impl core::ops::Neg for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(-self.to_radians())
        } else {
            Angle2D::from_degrees(-self.to_degrees())
        }
    }
}

impl Default for Angle2D {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::zero()
    }
}

impl core::fmt::Display for Angle2D {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.1 {
            return write!(f, "{}rad", self.to_radians());
        }
        write!(f, "{}°", self.to_degrees())
    }
}

impl<F: Into<float>> From<(F, bool)> for Angle2D {
    #[inline]
    #[must_use]
    fn from((f, r): (F, bool)) -> Self {
        if r {
            Self::from_radians(f.into())
        } else {
            Self::from_degrees(f.into())
        }
    }
}
