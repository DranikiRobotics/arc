//! Angle module for the arc crate
//!
//! Python identifier: `arc.math.angle`

use pyo3::prelude::*;

/// A Python class for 2D angles
///
/// This class is a wrapper around the [`libtrig::Angle2D`] type.
///
/// All methods and operators are implemented for this class.
///
/// If you want to use this class in your own code, you can use the [`libtrig::Angle2D`] type directly.
///
/// # Examples
///
/// ```py,no_run,ignore
/// from arc.math.angle import Angle
///
/// angle = Angle.from_degrees(90)
/// print(angle.radians())
/// ```
///
/// [`libtrig::Angle2D`]: ../../../libtrig/struct.Angle2D.html
#[pyclass]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Angle(pub(crate) libtrig::Angle2D);

#[pymethods]
impl Angle {
    #[inline]
    #[staticmethod]
    const fn from_degrees(degrees: libtrig::Degree64) -> Self {
        Self(libtrig::Angle2D::from_degrees(degrees))
    }
    #[inline]
    #[staticmethod]
    const fn from_radians(radians: libtrig::Radian64) -> Self {
        Self(libtrig::Angle2D::from_radians(radians))
    }
    #[inline]
    #[staticmethod]
    fn from_vec2d(vec: super::vec2d::Vec2D) -> Self {
        Self(libtrig::Angle2D::from(vec))
    }
    #[inline]
    #[staticmethod]
    const fn zero() -> Self {
        Self(libtrig::Angle2D::zero())
    }
    #[inline]
    fn degrees(&self) -> libtrig::Degree64 {
        self.0.to_degrees()
    }
    #[inline]
    fn radians(&self) -> libtrig::Radian64 {
        self.0.to_radians()
    }
    #[inline]
    fn sin(&self) -> libtrig::Float64 {
        libtrig::prelude!();
        self.0.sin()
    }
    #[inline]
    fn cos(&self) -> libtrig::Float64 {
        libtrig::prelude!();
        self.0.cos()
    }
    #[inline]
    fn sqrt(&self) -> libtrig::Float64 {
        libtrig::prelude!();
        self.0.sqrt()
    }
    #[inline]
    fn __add__(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }
    #[inline]
    fn __sub__(&self, other: &Self) -> Self {
        Self(self.0 - other.0)
    }
    #[inline]
    fn __mul__(&self, other: libtrig::Float64) -> Self {
        Self(self.0 * other)
    }
    #[inline]
    fn __truediv__(&self, other: libtrig::Float64) -> Self {
        Self(self.0 / other)
    }
    #[inline]
    fn __neg__(&self) -> Self {
        Self(-self.0)
    }
    #[inline]
    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
    #[inline]
    fn __ne__(&self, other: &Self) -> bool {
        self.0 != other.0
    }
    #[inline]
    fn __lt__(&self, other: &Self) -> bool {
        self.0 < other.0
    }
    #[inline]
    fn __le__(&self, other: &Self) -> bool {
        self.0 <= other.0
    }
    #[inline]
    fn __gt__(&self, other: &Self) -> bool {
        self.0 > other.0
    }
    #[inline]
    fn __ge__(&self, other: &Self) -> bool {
        self.0 >= other.0
    }
    #[inline]
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    #[inline]
    fn __repr__(&self) -> String {
        format!("Angle({:?})", self.0)
    }
}

impl From<libtrig::Angle2D> for Angle {
    fn from(angle: libtrig::Angle2D) -> Self {
        Self(angle)
    }
}

impl From<Angle> for libtrig::Angle2D {
    fn from(angle: Angle) -> Self {
        angle.0
    }
}

impl From<Angle> for libtrig::Vec2D {
    fn from(angle: Angle) -> Self {
        angle.0.into()
    }
}

impl core::ops::Deref for Angle {
    type Target = libtrig::Angle2D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Angle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The angle module
pub(crate) fn angle_submodule(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Angle>()?;

    Ok(())
}
