use crate::*;

/// A wrapper around a `float` value that represents an angle.
///
/// It can be created from either radians or degrees, and can be converted to either radians or degrees.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle2D(RadianOrDegree64, bool);

const DEG_TO_RAD_MULTIPLIER: Float64 = 0.0174532925199432957692369076848861271344287188854172545609;
const RAD_TO_DEG_MULTIPLIER: Float64 = 57.29577951308232087679815481410517033240547246656432154916;

impl Angle2D {
    /// Converts the given `deg` value to radians.
    #[inline]
    #[macros::func_mod(const => feature = "unstable")]
    pub fn deg_to_rad(deg: Degree64) -> Radian64 {
        deg * DEG_TO_RAD_MULTIPLIER
    }
    /// Converts the given `rad` value to degrees.
    #[inline]
    #[macros::func_mod(const => feature = "unstable")]
    pub fn rad_to_deg(rad: Radian64) -> Degree64 {
        rad * RAD_TO_DEG_MULTIPLIER
    }
}

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
    #[macros::func_mod(const => feature = "unstable")]
    pub fn to_radians(&self) -> Radian64 {
        if self.1 {
            return self.0;
        }
        Self::deg_to_rad(self.0)
    }
    /// Returns the value of the angle in degrees.
    #[inline]
    #[must_use]
    #[macros::func_mod(const => feature = "unstable")]
    pub fn to_degrees(&self) -> Degree64 {
        if !self.1 {
            return self.0;
        }
        Self::rad_to_deg(self.0)
    }
    /// If the angle is in degrees, converts it to radians.
    #[inline]
    #[macros::func_mod(const => feature = "unstable")]
    pub fn set_self_radians(&mut self) {
        if !self.1 {
            self.0 = Self::deg_to_rad(self.0);
            self.1 = true;
        }
    }
    /// If the angle is in radians, converts it to degrees.
    #[inline]
    #[macros::func_mod(const => feature = "unstable")]
    pub fn set_self_degrees(&mut self) {
        if self.1 {
            self.0 = Self::rad_to_deg(self.0);
            self.1 = false;
        }
    }
    /// Sets the mode of self.
    ///
    /// If `mode` is `true`, then the angle is in radians.
    /// If `mode` is `false`, then the angle is in degrees.
    ///
    /// This is unsafe because it does not convert the internal value.
    #[inline(always)]
    #[allow(unsafe_code)]
    #[macros::func_mod(const => feature = "unstable")]
    pub unsafe fn modify_self_mode_unchecked(&mut self, mode: bool) {
        self.1 = mode;
    }
    /// Sets the value of self.
    ///
    /// This is unsafe because it does not convert the internal value.
    #[inline(always)]
    #[allow(unsafe_code)]
    #[macros::func_mod(const => feature = "unstable")]
    pub unsafe fn modify_self_value_unchecked(&mut self, value: RadianOrDegree64) {
        self.0 = value;
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

impl From<u3> for Angle2D {
    #[inline]
    #[must_use]
    fn from(u: u3) -> Self {
        Self::from_radians(u.into())
    }
}

impl crate::traits::Sin<Float64> for Angle2D {
    #[inline]
    #[must_use]
    fn sin(&self) -> Float64 {
        l2math::sin(self.to_radians())
    }
}

impl crate::traits::Cos<Float64> for Angle2D {
    #[inline]
    #[must_use]
    fn cos(&self) -> Float64 {
        l2math::cos(self.to_radians())
    }
}

impl crate::traits::Sqrt<Float64> for Angle2D {
    #[inline]
    #[must_use]
    fn sqrt(&self) -> Float64 {
        self.0.sqrt()
    }
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
        self.to_radians().atan2(other.0)
    }
    #[inline]
    #[must_use]
    fn hypot(&self, other: Self) -> Float64 {
        self.to_radians().hypot(other.0)
    }
    #[inline]
    #[must_use]
    fn sin_cos(&self) -> (Float64, Float64) {
        self.to_radians().sin_cos()
    }
    #[inline]
    #[must_use]
    fn signof(&self, rhs: Self) -> Float64 {
        self.0.signof(rhs.0)
    }
    #[inline]
    #[must_use]
    fn tan(&self) -> Float64 {
        l2math::tan(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn asin(&self) -> Float64 {
        l2math::asin(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn acos(&self) -> Float64 {
        l2math::acos(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn atan(&self) -> Float64 {
        l2math::atan(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn sinh(&self) -> Float64 {
        l2math::sinh(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn cosh(&self) -> Float64 {
        l2math::cosh(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn tanh(&self) -> Float64 {
        l2math::tanh(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn asinh(&self) -> Float64 {
        l2math::asinh(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn acosh(&self) -> Float64 {
        l2math::acosh(self.to_radians())
    }
    #[inline]
    #[must_use]
    fn atanh(&self) -> Float64 {
        l2math::atanh(self.to_radians())
    }

    i!(floor ceil round trunc fract abs signum exp exp2 ln log2 log10 cbrt exp_m1 ln_1p);
}
