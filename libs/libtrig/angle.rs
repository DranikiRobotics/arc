use crate::*;

/// A wrapper around a `float` value that represents an angle.
///
/// It can be created from either radians or degrees, and can be converted to either radians or degrees.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle2D(RadianOrDegree64, bool);

impl Angle2D {
    /// Creates a new `Angle2D` from the given `value` and `is_radians` flag.
    #[inline]
    #[must_use]
    pub const fn new(value: RadianOrDegree64, is_radians: bool) -> Self {
        Self(value, is_radians)
    }
    /// Creates a new `Angle2D` from the given `value` in radians.
    #[inline]
    #[must_use]
    pub const fn from_radians(value: Radian64) -> Self {
        Self::new(value, true)
    }
    /// Creates a new `Angle2D` from the given `value` in degrees.
    #[inline]
    #[must_use]
    pub const fn from_degrees(value: Degree64) -> Self {
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
    pub fn to_radians(&self) -> Radian64 {
        if self.1 {
            self.0
        } else {
            self.0.to_radians()
        }
    }
    /// Returns the value of the angle in degrees.
    #[inline]
    #[must_use]
    pub fn to_degrees(&self) -> Degree64 {
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
    $OTHER = @OR Float64
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
    $OTHER = @OR Float64
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
    $OTHER = @OR Float64
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
    $OTHER = @OR Float64
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
    $OTHER = @OR Angle2D
)]
impl core::ops::Mul<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn mul(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() * rhs.to_radians())
        } else {
            Angle2D::from_degrees(self.to_degrees() * rhs.to_degrees())
        }
    }
}

#[macros::mass_impl(
    $THIS = @ORM Angle2D,
    $OTHER = @OR Float64
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
    $OTHER = @OR Angle2D
)]
impl core::ops::MulAssign<OTHER> for THIS {
    #[inline]
    fn mul_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() * rhs.to_radians();
        } else {
            self.0 = self.to_degrees() * rhs.to_degrees();
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR Float64
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
    $THIS = @ORM Angle2D,
    $OTHER = @OR Angle2D
)]
impl core::ops::Div<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn div(self, rhs: OTHER) -> Self::Output {
        if self.1 {
            Angle2D::from_radians(self.to_radians() / rhs.to_radians())
        } else {
            Angle2D::from_degrees(self.to_degrees() / rhs.to_degrees())
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR Float64
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
    $OTHER = @OR Angle2D
)]
impl core::ops::DivAssign<OTHER> for THIS {
    #[inline]
    fn div_assign(&mut self, rhs: OTHER) {
        if self.1 {
            self.0 = self.to_radians() / rhs.to_radians();
        } else {
            self.0 = self.to_degrees() / rhs.to_degrees();
        }
    }
}

#[macros::mass_impl(
    $THIS = @OM Angle2D,
    $OTHER = @OR Float64
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

impl<F: Into<Float64>> From<(F, bool)> for Angle2D {
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

impl crate::traits::Number for Angle2D {}

macro_rules! i {
    ($n:ident $($t:tt)*) => (
        #[inline]
        #[must_use]
        fn $n(&self) -> Float64 {
            self.0.$n()
        }
        i!($($t)*);
    );
    () => ();
}

impl crate::traits::Float<Float64> for Angle2D {
    #[inline]
    #[must_use]
    fn mul_add(&self, a: Self, b: Self) -> Float64 {
        self.0.mul_add(a.0, b.0)
    }
    #[inline]
    #[must_use]
    fn div_euclid(&self, rhs: Self) -> Float64 {
        self.0.div_euclid(rhs.0)
    }
    #[inline]
    #[must_use]
    fn rem_euclid(&self, rhs: Self) -> Float64 {
        self.0.rem_euclid(rhs.0)
    }
    #[inline]
    #[must_use]
    fn powi(&self, n: Int) -> Float64 {
        self.0.powi(n)
    }
    #[inline]
    #[must_use]
    fn powf(&self, n: Self) -> Float64 {
        self.0.powf(n.0)
    }
    #[inline]
    #[must_use]
    fn log(&self, base: Self) -> Float64 {
        self.0.log(base.0)
    }
    #[inline]
    #[must_use]
    fn atan2(&self, other: Self) -> Float64 {
        self.0.atan2(other.0)
    }
    #[inline]
    #[must_use]
    fn hypot(&self, other: Self) -> Float64 {
        self.0.hypot(other.0)
    }
    #[inline]
    #[must_use]
    fn sin_cos(&self) -> (Float64, Float64) {
        self.0.sin_cos()
    }
    #[inline]
    #[must_use]
    fn signof(&self, rhs: Self) -> Float64 {
        self.0.signof(rhs.0)
    }

    i!(floor ceil round trunc fract abs signum sqrt exp exp2 ln log2 log10 cbrt
       sin cos tan asin acos atan exp_m1 ln_1p sinh cosh tanh asinh acosh atanh);
}
