//! Math module for the arc crate
//!
//! Python identifier: `arc.math`

use pyo3::prelude::*;

pub mod angle;
pub mod pos2d;
pub mod vec2d;

/// The math module.
pub(crate) fn math_submodule(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    crate::pymod!(angle -> angle::angle_submodule, "arc.math.angle", py, m);
    crate::pymod!(pos2d -> pos2d::pos2d_submodule, "arc.math.pos2d", py, m);
    crate::pymod!(vec2d -> vec2d::vec2d_submodule, "arc.math.vec2d", py, m);

    m.add_class::<angle::Angle>()?;
    m.add_class::<pos2d::Pos2D>()?;
    m.add_class::<vec2d::Vec2D>()?;
    Ok(())
}
