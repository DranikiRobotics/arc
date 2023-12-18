//! Pos2D module for the arc crate
//!
//! Python identifier: `arc.math.pos2d`

use libtrig::{Float64, Radian64};
use pyo3::prelude::*;

/// The Pos2D class.
#[pyclass]
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Pos2D(libtrig::Pos2D);

#[pymethods]
impl Pos2D {
    #[inline]
    #[staticmethod]
    fn from_angle_and_pos(rot: super::angle::Angle, pos: super::vec2d::Vec2D) -> Self {
        Self(libtrig::Pos2D::new(pos.into(), rot.into()))
    }
    #[inline]
    #[staticmethod]
    fn from_xyr(x: Float64, y: Float64, radians: Radian64) -> Self {
        Self(libtrig::Pos2D::new(
            libtrig::Coord2D::new(x, y),
            libtrig::Angle2D::from_radians(radians),
        ))
    }
    #[inline]
    #[staticmethod]
    fn from_xy(x: Float64, y: Float64) -> Self {
        Self(libtrig::Pos2D::new(
            libtrig::Coord2D::new(x, y),
            libtrig::Angle2D::zero(),
        ))
    }
    #[inline]
    #[staticmethod]
    fn from_vec2d(vec: super::vec2d::Vec2D) -> Self {
        Self(libtrig::Pos2D::new(
            vec.into(), libtrig::Angle2D::zero()
        ))
    }
    #[inline]
    #[staticmethod]
    fn from_angle(angle: super::angle::Angle) -> Self {
        Self(libtrig::Pos2D::new(
            libtrig::Coord2D::origin(),
            libtrig::Angle2D::from(angle),
        ))
    }
    #[inline]
    #[staticmethod]
    fn zero() -> Self {
        Self(libtrig::Pos2D::zero())
    }
    #[inline]
    #[getter]
    fn x(&self) -> Float64 {
        self.0.x()
    }
    #[inline]
    #[getter]
    fn y(&self) -> Float64 {
        self.0.y()
    }
    #[inline]
    #[getter]
    fn angle(&self) -> super::angle::Angle {
        super::angle::Angle(self.0.angle())
    }
    #[inline]
    fn r#move(&mut self, delta: Self) {
        self.0.translate(delta.0)
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
}

pub(crate) fn pos2d_submodule(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Pos2D>()?;

    Ok(())
}
