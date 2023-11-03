use crate::traits::Float;
use crate::*;

/// A wrapper around a `float` value that represents a 2D coordinate.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coord2D {
    /// The x coordinate.
    pub x: float,
    /// The y coordinate.
    pub y: float,
}

impl Coord2D {
    /// Creates a new `Vec2D` from the given `x` and `y` values.
    #[inline]
    #[must_use]
    pub const fn new(x: float, y: float) -> Self {
        Self { x, y }
    }
    /// Creates a new `Vec2D` located at the origin.
    #[inline]
    #[must_use]
    pub const fn origin() -> Self {
        Self::new(0.0, 0.0)
    }
    /// Rotates the vector by the given angle in radians.
    #[must_use]
    pub fn rotate_by(&self, angle: Angle2D) -> Self {
        let angle = angle.to_radians();
        let (sin, cos) = angle.sin_cos();
        Self::new(
            self.x * cos - self.y * sin,
            self.x * sin + self.y * cos,
        )
    }
    /// Returns the angle of the vector in radians.
    #[inline]
    #[must_use]
    pub fn angle(&self) -> Angle2D {
        Angle2D::from_radians(self.y.atan2(self.x))
    }
    /// Returns the inverted position.
    ///
    /// Same as rotating the position by 180 degrees.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        -self
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x + rhs.x(), self.y + rhs.y())
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.x += rhs.x();
        self.y += rhs.y();
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x - rhs.x(), self.y - rhs.y())
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x - rhs.x, self.y - rhs.y)
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.x -= rhs.x();
        self.y -= rhs.y();
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @OR float
)]
impl core::ops::Mul<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn mul(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x * rhs, self.y * rhs)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @OR Vec2D
)]
impl core::ops::Mul<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn mul(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x * rhs.x(), self.y * rhs.y())
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @OR float
)]
impl core::ops::MulAssign<OTHER> for THIS {
    #[inline]
    fn mul_assign(&mut self, rhs: OTHER) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @OR Vec2D
)]
impl core::ops::MulAssign<OTHER> for THIS {
    #[inline]
    fn mul_assign(&mut self, rhs: OTHER) {
        self.x *= rhs.x();
        self.y *= rhs.y();
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @OR float
)]
impl core::ops::Div<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn div(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x / rhs, self.y / rhs)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Coord2D,
    $OTHER = @OR Vec2D
)]
impl core::ops::Div<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn div(self, rhs: OTHER) -> Self::Output {
        Coord2D::new(self.x / rhs.x(), self.y / rhs.y())
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @OR float
)]
impl core::ops::DivAssign<OTHER> for THIS {
    #[inline]
    fn div_assign(&mut self, rhs: OTHER) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Coord2D,
    $OTHER = @OR Vec2D
)]
impl core::ops::DivAssign<OTHER> for THIS {
    #[inline]
    fn div_assign(&mut self, rhs: OTHER) {
        self.x /= rhs.x();
        self.y /= rhs.y();
    }
}

#[macros::mass_impl($THIS = @ORM Coord2D)]
impl core::ops::Neg for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Coord2D::new(-self.x, -self.y)
    }
}

impl core::fmt::Display for Coord2D {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl Default for Coord2D {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::origin()
    }
}

impl<F: Into<float>> From<(F, F)> for Coord2D {
    #[inline]
    #[must_use]
    fn from((x, y): (F, F)) -> Self {
        Self::new(x.into(), y.into())
    }
}

impl From<Coord2D> for (float, float) {
    #[inline]
    #[must_use]
    fn from(Coord2D {x, y}: Coord2D) -> Self {
        (x, y)
    }
}
