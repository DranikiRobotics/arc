#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

#[doc(hidden)]
pub use arc_robot_core;

pub mod __init__;
mod macros;
pub mod threadsafe;

pub use threadsafe::{ThreadSafe, ThreadSafeError};

/// A trait for hardware components that can be used in the robot.
pub trait PyWrappedComponent<Input> {
    /// The type that holds the hardware component.
    ///
    /// This type isn't required to be `Send` or `Sync`.
    ///
    /// The holder is what will be written to by the python thread.
    type Holder;
    /// Creates a new hardware component.
    ///
    /// This function should be called before the python main thread is started.
    fn new(hardware: Input) -> crate::ThreadSafe<Self::Holder>;
    /// Wraps the hardware component in a `ThreadSafe` type.
    ///
    /// The wrapper is what will be read from by the python thread.
    fn wrap(hardware_component: &crate::ThreadSafe<Self::Holder>) -> Self;
}

/// Internal function to translate a static string into a PyIOError.
#[doc(hidden)]
fn make_err(e: &'static str) -> pyo3::PyErr {
    pyo3::PyErr::new::<pyo3::exceptions::PyIOError, _>(e)
}

/// A type that represents a python function.
/// 
/// This type is used to call python functions from rust.
/// 
/// # Example
/// 
/// ```rust
/// # use pyo3::prelude::*;
/// # use arc_pylib as pylib;
/// ```
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PyFunction(pyo3::Py<pyo3::PyAny>);

impl PyFunction {
    /// Calls the python function with the given arguments.
    pub fn call(&self,
        py: pyo3::Python<'_>,
        args: impl pyo3::IntoPy<pyo3::Py<pyo3::types::PyTuple>>
    ) -> pyo3::PyResult<pyo3::PyObject> {
        self.0.call1(py, args)
    }
}
