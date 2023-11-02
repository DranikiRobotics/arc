use crate::{Angle, Float};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2D(Float, Float);

impl Vec2D {
    /// Creates a new `Vec2D` from the given `x` and `y` values.
    #[inline]
    #[must_use]
    pub const fn new(x: Float, y: Float) -> Self {
        Self(x, y)
    }
    /// Creates a new `Vec2D` from the given `x` and `y` values.
    #[inline]
    #[must_use]
    pub const fn origin() -> Self {
        Self(0.0, 0.0)
    }
    /// Returns the x component of the vector.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> Float {
        self.0
    }
    /// Returns the y component of the vector.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> Float {
        self.1
    }
    /// Rotates the vector by the given angle in radians.
    #[must_use]
    pub fn rotate_by(&self, angle: Angle) -> Self {
        let angle = angle.to_radians();
        let (sin, cos) = angle.sin_cos();
        Self(
            self.x() * cos - self.y() * sin,
            self.x() * sin + self.y() * cos,
        )
    }
    /// Returns the angle of the vector in radians.
    #[inline]
    #[must_use]
    pub fn angle(&self) -> Angle {
        Angle::from_radians(self.y().atan2(self.x()))
    }
    /// Returns the inverted vector.
    ///
    /// Same as rotating the vector by 180 degrees.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        -*self
    }
    /// Returns the dot product of the vector and the given vector.
    #[inline]
    #[must_use]
    pub fn dot(&self, other: Self) -> Float {
        self.x() * other.x() + self.y() * other.y()
    }
    /// Returns the magnitude of the vector.
    #[inline]
    #[must_use]
    pub fn magnitude(&self) -> Float {
        self.dot(*self).sqrt()
    }
    /// Returns the normalized vector.
    #[inline]
    #[must_use]
    pub fn normalize(&mut self) {
        // let magnitude = self.magnitude();
        // self /= magnitude;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec2D,
    $OTHER = @ORM Vec2D
)]
impl std::ops::Add<OTHER> for THIS {
    type Output = Vec2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Vec2D(self.x() + rhs.x(), self.y() + rhs.y())
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec2D,
    $OTHER = @ORM Vec2D
)]
impl std::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.0 += rhs.x();
        self.1 += rhs.y();
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec2D,
    $OTHER = @ORM Vec2D
)]
impl std::ops::Sub<OTHER> for THIS {
    type Output = Vec2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Vec2D(self.x() - rhs.x(), self.y() - rhs.y())
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec2D,
    $OTHER = @ORM Vec2D
)]
impl std::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.0 -= rhs.x();
        self.1 -= rhs.y();
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec2D,
    $OTHER = @OR Float
)]
impl std::ops::Mul<OTHER> for THIS {
    type Output = Vec2D;
    #[inline]
    #[must_use]
    fn mul(self, rhs: OTHER) -> Self::Output {
        Vec2D(self.x() * rhs, self.y() * rhs)
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec2D,
    $OTHER = @OR Float
)]
impl std::ops::MulAssign<OTHER> for THIS {
    #[inline]
    fn mul_assign(&mut self, rhs: OTHER) {
        self.0 *= rhs;
        self.1 *= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Vec2D,
    $OTHER = @OR Float
)]
impl std::ops::Div<OTHER> for THIS {
    type Output = Vec2D;
    #[inline]
    #[must_use]
    fn div(self, rhs: OTHER) -> Self::Output {
        Vec2D(self.x() / rhs, self.y() / rhs)
    }
}

#[macros::mass_impl(
    $THIS = @OM Vec2D,
    $OTHER = @OR Float
)]
impl std::ops::DivAssign<OTHER> for THIS {
    #[inline]
    fn div_assign(&mut self, rhs: OTHER) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

#[macros::mass_impl($THIS = @ORM Vec2D)]
impl std::ops::Neg for THIS {
    type Output = Vec2D;
    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Vec2D(-self.x(), -self.y())
    }
}

impl std::fmt::Display for Vec2D {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x(), self.y())
    }
}

impl Default for Vec2D {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::origin()
    }
}

impl From<(Float, Float)> for Vec2D {
    #[inline]
    #[must_use]
    fn from((x, y): (Float, Float)) -> Self {
        Self::new(x, y)
    }
}

impl From<Vec2D> for (Float, Float) {
    #[inline]
    #[must_use]
    fn from(Vec2D(x, y): Vec2D) -> Self {
        (x, y)
    }
}
