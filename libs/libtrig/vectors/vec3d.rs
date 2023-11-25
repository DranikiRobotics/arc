use crate::*;

/// A 3D vector.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3D(Float64, Float64, Float64);

impl Vec3D {
    /// Creates a new `Vec3D` from the given `x`, `y` and `z` values.
    #[inline]
    #[must_use]
    pub const fn new(x: Float64, y: Float64, z: Float64) -> Self {
        Self(x, y, z)
    }
    /// Creates a new `Vec3D` located at the origin.
    #[inline]
    #[must_use]
    pub const fn origin() -> Self {
        Self(0.0, 0.0, 0.0)
    }
    /// Returns the x component of the vector.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> Float64 {
        self.0
    }
    /// Returns the y component of the vector.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> Float64 {
        self.1
    }
    /// Returns the z component of the vector.
    #[inline]
    #[must_use]
    pub const fn z(&self) -> Float64 {
        self.2
    }
    /// Returns the inverted vector.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        -self
    }
    /// Returns the dot product of the vector and the given vector.
    #[inline]
    #[must_use]
    pub fn dot(&self, other: Self) -> Float64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }
    /// Returns the magnitude of the vector.
    #[inline]
    #[must_use]
    pub fn magnitude(&self) -> Float64 {
        #[allow(unused_imports)]
        use crate::traits::*;
        self.dot(*self).sqrt()
    }
    /// Normalizes this vector.
    #[inline]
    pub fn normalize(&mut self) {
        let magnitude = self.magnitude();
        self.0 /= magnitude;
        self.1 /= magnitude;
        self.2 /= magnitude;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec3D,
    $OTHER = @ORM Vec3D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Vec3D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Vec3D(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec3D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Vec3D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Vec3D(self.x() + rhs.x(), self.y() + rhs.y(), self.z())
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec3D,
    $OTHER = @ORM Vec3D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.0 += rhs.x();
        self.1 += rhs.y();
        self.2 += rhs.z();
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec3D,
    $OTHER = @ORM Vec3D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Vec3D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Vec3D(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec3D,
    $OTHER = @ORM Vec3D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.0 -= rhs.x();
        self.1 -= rhs.y();
        self.2 -= rhs.z();
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec3D,
    $OTHER = @OR Float64
)]
impl core::ops::Mul<OTHER> for THIS {
    type Output = Vec3D;
    #[inline]
    #[must_use]
    fn mul(self, rhs: OTHER) -> Self::Output {
        Vec3D(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec3D,
    $OTHER = @OR Float64
)]
impl core::ops::MulAssign<OTHER> for THIS {
    #[inline]
    fn mul_assign(&mut self, rhs: OTHER) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec3D,
    $OTHER = @OR Float64
)]
impl core::ops::Div<OTHER> for THIS {
    type Output = Vec3D;
    #[inline]
    #[must_use]
    fn div(self, rhs: OTHER) -> Self::Output {
        Vec3D(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec3D,
    $OTHER = @OR Float64
)]
impl core::ops::DivAssign<OTHER> for THIS {
    #[inline]
    fn div_assign(&mut self, rhs: OTHER) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

#[macros::mass_impl($THIS = @ORM Vec3D)]
impl core::ops::Neg for THIS {
    type Output = Vec3D;
    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Vec3D(-self.x(), -self.y(), -self.z())
    }
}

impl core::fmt::Display for Vec3D {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

impl Default for Vec3D {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::origin()
    }
}

impl<F: Into<Float64>> From<(F, F, F)> for Vec3D {
    #[inline]
    #[must_use]
    /// Converts a tuple of 3 Floats into a `Vec3D`.
    fn from((x, y, z): (F, F, F)) -> Self {
        Self::new(x.into(), y.into(), z.into())
    }
}

impl From<Vec3D> for (Float64, Float64, Float64) {
    #[inline]
    #[must_use]
    fn from(Vec3D(x, y, z): Vec3D) -> Self {
        (x, y, z)
    }
}
