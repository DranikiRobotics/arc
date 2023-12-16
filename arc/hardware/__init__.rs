use pyo3::prelude::*;

pub mod gamepad;

#[pymodule]
pub fn hardware(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Modules
    m.add_wrapped(pyo3::wrap_pymodule!(gamepad::gamepad))?;

    // Their classes
    m.add_class::<gamepad::Gamepad>()?;

    Ok(())
}
