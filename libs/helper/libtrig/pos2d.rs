use l2math::{Float64, Radian64};

use crate::{Angle2D, Coord2D, Vec2D};

/// A 2D position with rotation.
///
/// This type is a wrapper around the [`Coord2D`] and [`Angle2D`] types.
///
/// All logical operations are implemented for this type.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Pos2D {
    pos: Coord2D,
    rot: Angle2D,
}

impl core::fmt::Display for Pos2D {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{};{} {}", self.x(), self.y(), self.angle())
    }
}

impl Default for Pos2D {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::zero()
    }
}

impl Pos2D {
    /// Create a new `Pos2D` from a `Coord2D` and an `Angle2D`.
    #[inline]
    #[must_use]
    pub const fn new(pos: Coord2D, rot: Angle2D) -> Self {
        Self { pos, rot }
    }
    /// Create a new `Pos2D` positioned at the origin with no rotation.
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(Coord2D::origin(), Angle2D::zero())
    }
    /// Returns the x coordinate of the `Pos2D`.
    #[inline]
    #[must_use]
    pub const fn x(&self) -> Float64 {
        self.pos.x()
    }
    /// Returns the y coordinate of the `Pos2D`.
    #[inline]
    #[must_use]
    pub const fn y(&self) -> Float64 {
        self.pos.y()
    }
    /// Returns the angle of rotation of the `Pos2D`.
    #[inline]
    #[must_use]
    pub const fn angle(&self) -> Angle2D {
        self.rot
    }
    /// Moves the `Pos2D` by the given delta `Pos2D`
    #[inline]
    pub fn translate(&mut self, delta: Pos2D) {
        self.pos += delta.pos;
        self.rot += delta.rot;
    }
}

impl From<Coord2D> for Pos2D {
    fn from(pos: Coord2D) -> Self {
        Self::new(pos, Angle2D::zero())
    }
}

impl From<Vec2D> for Pos2D {
    fn from(vec: Vec2D) -> Self {
        Self::new(vec.into(), Angle2D::zero())
    }
}

impl From<Angle2D> for Pos2D {
    fn from(rot: Angle2D) -> Self {
        Self::new(Coord2D::origin(), rot)
    }
}

impl From<(Float64, Float64, Radian64)> for Pos2D {
    fn from((x, y, rot): (Float64, Float64, Radian64)) -> Self {
        Self::new(Coord2D::new(x, y), Angle2D::from_radians(rot))
    }
}

impl From<(Float64, Float64)> for Pos2D {
    fn from((x, y): (Float64, Float64)) -> Self {
        Self::new(Coord2D::new(x, y), Angle2D::zero())
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        self.pos + rhs
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        self.rot + rhs
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::Add<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn add(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(self.pos + rhs.pos, self.rot + rhs.rot)
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.pos += rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.rot += rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::AddAssign<OTHER> for THIS {
    #[inline]
    fn add_assign(&mut self, rhs: OTHER) {
        self.pos += rhs.pos;
        self.rot += rhs.rot;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Coord2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        self.pos - rhs
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D,
    $OTHER = @ORM Angle2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Angle2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        self.rot - rhs
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::Sub<OTHER> for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn sub(self, rhs: OTHER) -> Self::Output {
        Pos2D::new(self.pos - rhs.pos, self.rot - rhs.rot)
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Vec2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.pos -= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @OR Angle2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.rot -= rhs;
    }
}

#[macros::mass_impl(
    $THIS = @OM Pos2D,
    $OTHER = @ORM Pos2D
)]
impl core::ops::SubAssign<OTHER> for THIS {
    #[inline]
    fn sub_assign(&mut self, rhs: OTHER) {
        self.pos -= rhs.pos;
        self.rot -= rhs.rot;
    }
}

#[macros::mass_impl(
    $THIS = @ORM Pos2D
)]
impl core::ops::Neg for THIS {
    type Output = Pos2D;
    #[inline]
    #[must_use]
    fn neg(self) -> Self::Output {
        Pos2D::new(-self.pos, -self.rot)
    }
}

impl PartialEq for Pos2D {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.rot == other.rot
    }
}

impl Eq for Pos2D {}
