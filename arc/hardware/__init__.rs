//! Hardware module for the arc crate
//!
//! Python identifier: `arc.hardware`

use pyo3::prelude::*;

pub mod gamepad;

/// The hardware module.
pub(crate) fn hardware_submodule(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    crate::pymod!(gamepad -> gamepad::gamepad_submodule, "arc.hardware.gamepad", py, m);

    m.add_class::<gamepad::Gamepad>()?;
    m.add_class::<gamepad::GamepadDpad>()?;
    m.add_class::<gamepad::GamepadStick>()?;

    Ok(())
}
