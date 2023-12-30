#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

#[cfg(any(
    not(feature = "dranik-only-builtins"),
    not(feature = "math")
))]
compile_error!("The `dranik-only-builtins` feature is required to use this crate, as well as the `math` feature.");

pub mod __init__;
#[doc(hidden)]
pub mod macros;

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

impl dranikcore::prelude::RobotConfig for __dranik_config {
    fn python_preload() {
        use crate::__init__::arc as __arc_pylib;
        pyo3::append_to_inittab!(__arc_pylib);
    }
    type Args = __init__::Op;
    fn build_python_main_function_args<'a>(
        _py: &pyo3::Python<'_>,
        op: &dranikcore::RuntimeOp
    ) -> (Self::Args, Option<&'a pyo3::types::PyDict>) {
        (__init__::Op::from(op.clone()), None)
    }
}
