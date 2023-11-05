use crate::*;

/// A 2D position.
/// 
/// Contains a `Coord2D` and an `Angle2D`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Pos2D(Coord2D, Angle2D);

impl Pos2D {
    /// Creates a new `Pos2D` from the given `coords` and `angle`.
    #[inline]
    #[must_use]
    pub const fn new(coords: Coord2D, angle: Angle2D) -> Self {
        Self(coords, angle)
    }
    /// Creates a new `Pos2D` at the origin, with an angle of `0.0` radians.
    #[inline]
    #[must_use]
    pub const fn origin() -> Self {
        Self(Coord2D::origin(), Angle2D::zero())
    }
    /// Returns the x coordinate of the position.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> Float64 {
        self.0.x
    }
    /// Sets the x coordinate of the position.
    #[inline]
    #[allow(non_snake_case)]
    pub const fn setX(&mut self, x: Float64) {
        self.0.x = x;
    }
    /// Returns the y coordinate of the position.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> Float64 {
        self.0.y
    }
    /// Sets the y coordinate of the position.
    #[inline]
    #[allow(non_snake_case)]
    pub const fn setY(&mut self, y: Float64) {
        self.0.y = y;
    }
    /// Rotates the position by the given angle in radians.
    #[must_use]
    pub fn rotate_by(&self, angle: Angle2D) -> Self {
        Self(self.0.rotate_by(angle), self.1 + angle)
    }
    /// Returns the facing angle.
    #[inline]
    #[must_use]
    pub fn angle(&self) -> Angle2D {
        self.1
    }
    /// Sets the facing angle.
    #[inline]
    #[allow(non_snake_case)]
    pub const fn setAngle(&mut self, angle: Angle2D) {
        self.1 = angle;
    }
    /// Returns the inverted Position.
    ///
    /// Same as rotating the position by 180 degrees.
    #[inline]
    #[must_use]
    pub fn inverse(&self) -> Self {
        -self
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(Coord2D::new(self.0.x + rhs.x(), self.0.y + rhs.y()), self.1)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(Coord2D::new(self.0.x + rhs.x, self.0.y + rhs.y), self.1)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(Coord2D::new(self.0.x + rhs.0.x, self.0.y + rhs.0.y), self.1 + rhs.1)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(self.0, self.1 + rhs)
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.0 += rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.0 += rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.1 += rhs;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(Coord2D::new(self.0.x - rhs.x(), self.0.y - rhs.y()), self.1)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(Coord2D::new(self.0.x - rhs.x, self.0.y - rhs.y), self.1)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(Coord2D::new(self.0.x - rhs.0.x, self.0.y - rhs.0.y), self.1 - rhs.1)
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(self.0, self.1 - rhs)
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.0 -= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Coord2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.0 -= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @OR Angle2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.1 -= rhs;
    }
}

#[macros::mass_impl($THIS = @ORM Pos2D)]
impl core::ops::Neg for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Pos2D::new(-self.0, -self.1)
    }
}

impl core::fmt::Display for Pos2D {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}; {}", self.0, self.1)
    }
}

impl Default for Pos2D {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::origin()
    }
}

impl<C: Into<Coord2D>, A: Into<Angle2D>> From<(C, A)> for Pos2D {
    #[inline]
    #[must_use]
    fn from((c, a): (C, A)) -> Self {
        Self::new(c.into(), a.into())
    }
}

impl<F: Into<Float64>> From<(F, F, F, bool)> for Pos2D {
    #[inline]
    #[must_use]
    fn from(i: (F, F, F, bool)) -> Self {
        Self::new(
            Coord2D::from((i.0, i.1)),
            Angle2D::from((i.2, i.3))
        )
    }
}

impl<F: Into<Float64>> From<(F, F, F)> for Pos2D {
    /// Creates a new `Pos2D` from the given `coords` and `angle`.
    #[inline]
    #[must_use]
    fn from(i: (F, F, F)) -> Self {
        Self::from((i.0, i.1, i.2, false))
    }
}
