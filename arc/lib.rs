#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

pub mod __init__;
#[doc(hidden)]
pub mod macros;

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

#[doc(hidden)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy)]
pub struct __dranik_config;

impl dranikcore::config::RobotConfig for __dranik_config {
    fn python_preload() {
        use crate::__init__::arc as __arc_pylib;
        pyo3::append_to_inittab!(__arc_pylib);
    }
    type Args = __init__::Op;
    fn build_python_main_function_args<'a>(
        py: &pyo3::Python<'_>,
        io: &dranikcore::io::IO
    ) -> (Self::Args, Option<&'a pyo3::types::PyDict>) {
        (__init__::Op::from(io), None)
    }
}
