//! Vec2D module for the arc crate
//!
//! Python identifier: `arc.math.vec2d`

use pyo3::prelude::*;

/// A Python class for 2D vectors
///
/// This class is a wrapper around the [`libtrig::Vec2D`] type.
///
/// All methods and operators are implemented for this class.
///
/// If you want to use this class in your own code, you can use the [`libtrig::Vec2D`] type directly.
///
/// # Examples
///
/// ```py,no_run,ignore
/// from arc.math.vec2d import Vec2D
///
/// vec = Vec2D.from_xy(3, 4)
/// print(vec.magnitude())
/// ```
///
/// [`libtrig::Vec2D`]: ../../../libtrig/struct.Vec2D.html
#[pyclass]
#[derive(Debug, Clone)]
pub struct Vec2D(libtrig::Vec2D);

#[pymethods]
impl Vec2D {
    #[inline]
    #[staticmethod]
    fn from_angle(angle: super::angle::Angle) -> Self {
        Self(libtrig::Vec2D::from(angle))
    }
    #[inline]
    #[staticmethod]
    const fn from_xy(x: libtrig::Float64, y: libtrig::Float64) -> Self {
        Self(libtrig::Vec2D::new(x, y))
    }
    #[inline]
    #[getter]
    fn x(&self) -> libtrig::Float64 {
        self.0.x()
    }
    #[inline]
    #[getter]
    fn y(&self) -> libtrig::Float64 {
        self.0.y()
    }
    #[inline]
    fn angle(&self) -> super::angle::Angle {
        super::angle::Angle(self.0.into())
    }
    #[inline]
    fn magnitude(&self) -> libtrig::Float64 {
        self.0.magnitude()
    }
    #[inline]
    fn normalize(&self) -> Self {
        Self(self.0.normalize())
    }
    #[inline]
    fn dot(&self, other: Self) -> libtrig::Float64 {
        libtrig::prelude!();
        self.0.dot(other.0)
    }
    #[inline]
    fn __add__(&self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
    #[inline]
    fn __sub__(&self, other: Self) -> Self {
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
    fn __eq__(&self, other: Self) -> bool {
        self.0 == other.0
    }
    #[inline]
    fn __ne__(&self, other: Self) -> bool {
        self.0 != other.0
    }
    #[inline]
    fn __lt__(&self, other: Self) -> bool {
        self.0 < other.0
    }
    #[inline]
    fn __le__(&self, other: Self) -> bool {
        self.0 <= other.0
    }
    #[inline]
    fn __gt__(&self, other: Self) -> bool {
        self.0 > other.0
    }
    #[inline]
    fn __ge__(&self, other: Self) -> bool {
        self.0 >= other.0
    }
    #[inline]
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }
    #[inline]
    fn __repr__(&self) -> String {
        format!("Vec2D({:?})", self.0)
    }
    #[inline]
    fn __len__(&self) -> usize {
        self.magnitude().round() as usize
    }
}

impl From<libtrig::Vec2D> for Vec2D {
    fn from(vec: libtrig::Vec2D) -> Self {
        Self(vec)
    }
}

impl From<Vec2D> for libtrig::Vec2D {
    fn from(vec: Vec2D) -> Self {
        vec.0
    }
}

impl From<Vec2D> for libtrig::Angle2D {
    fn from(vec: Vec2D) -> Self {
        vec.0.into()
    }
}

impl From<Vec2D> for libtrig::Coord2D {
    fn from(vec: Vec2D) -> Self {
        vec.0.into()
    }
}

impl core::ops::Deref for Vec2D {
    type Target = libtrig::Vec2D;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::ops::DerefMut for Vec2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// The Vec2D module
pub(crate) fn vec2d_submodule(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Vec2D>()?;

    Ok(())
}
