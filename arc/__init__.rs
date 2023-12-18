//! The rust bindings for the root of the arc library
//!
//! This module contains the [`Op`] struct, which is the main struct that
//! is used to access the robot's hardware.
//!
//! [`Op`]: struct.Op.html

use crate::threadsafe::{self, ThreadSafe};
use pyo3::prelude::*;

/// Hardware submodule
#[path = "hardware/__init__.rs"]
pub mod hardware;

/// Math submodule
#[path = "math/__init__.rs"]
pub mod math;

/// The struct that actually contains the necessary data for the op mode
/// to run.
///
/// This struct should only be used for mutating the data outside of the
/// op mode thread. For reading the up to date data, use the `Op` struct.
#[derive(Debug)]
pub struct OpHolder {
    running: threadsafe::ThreadSafeBool,
    gamepad: hardware::gamepad::Gamepad,
    start_time: std::time::Instant,
}

impl OpHolder {
    /// Returns whether the op mode is running
    ///
    /// This call aquires a lock on the data
    pub fn running(&self) -> bool {
        match self.running.get() {
            Ok(r) => **r,
            Err(_) => false,
        }
    }
    /// Returns a reference to the gamepad
    ///
    /// This call aquires a lock on the data
    pub fn gamepad(&self) -> &hardware::gamepad::Gamepad {
        &self.gamepad
    }
    /// Stops the op mode
    ///
    /// DO NOT CALL THIS FROM THE PYTHON THREAD
    pub fn stop(&self) -> core::result::Result<(), &'static str> {
        self.running.get_mut()?.set(false);
        Ok(())
    }
    /// Returns the running time of the op mode
    ///
    /// This call does not aquire a lock on the data,
    /// nor does it need to.
    pub fn running_time(&self) -> core::time::Duration {
        std::time::Instant::now() - self.start_time
    }
}

threadsafe::thread_safe!(OpHolder);

/// The struct that is used to access the data in the op mode
///
/// This struct internally uses a `ThreadSafe` to access the data.
/// So feel free to clone it. It is also `Send` and `Sync`.
///
/// This struct should be used for reading the data in the op mode thread.
/// For mutating the data outside of the op mode thread, use the `OpHolder`
/// struct.
///
/// # Example
///
/// ```rust
/// # use pyo3::prelude::*;
/// # use arc_pylib as pylib;
/// # use arc_pylib::arc_robot_hardware as hardware;
/// # use hardware::IO_OK;
/// # use pylib::PyWrappedComponent as _;
/// # use pylib::__init__::Op;
/// # use pylib::setup_wrapped_component;
/// # fn main() -> hardware::Result {
/// # let (gamepad, gamepad_wrapper) = setup_wrapped_component!(
/// # pylib::arc_robot_hardware::gamepad::impls::LogitechF310::default();
/// # pylib::__init__::hardware::gamepad::Gamepad
/// # );
/// let (op, op_wrapper) = setup_wrapped_component!(gamepad_wrapper; Op);
///
/// // IO Thread
/// let time = op.get()?.running_time();
/// if time.as_secs() >= 30 { // If we want to stop after 30 seconds
///    op.get_mut()?.stop()?;
/// }
/// # IO_OK
/// # }
/// ```
#[pyclass]
#[derive(Debug, Clone)]
pub struct Op(ThreadSafe<OpHolder>);

impl crate::PyWrappedComponent<hardware::gamepad::Gamepad> for Op {
    type Holder = OpHolder;
    fn new(gamepad: hardware::gamepad::Gamepad) -> ThreadSafe<Self::Holder> {
        ThreadSafe::new(OpHolder {
            start_time: std::time::Instant::now(),
            running: true.into(),
            gamepad,
        })
    }
    fn wrap(gamepad: &ThreadSafe<Self::Holder>) -> Self {
        Self(gamepad.clone())
    }
}

impl Op {
    /// Returns a new [`Gamepad`] struct that is a clone of the one in the op mode.
    ///
    /// (It's an `Arc` so it's cheap to clone)
    ///
    /// [`Gamepad`]: _hardware/gamepad/struct.Gamepad.html
    pub fn get_gamepad(&self) -> core::result::Result<hardware::gamepad::Gamepad, &'static str> {
        self.0.get().map(|g| g.gamepad().clone())
    }
    /// Returns whether the op mode is running
    ///
    /// This call aquires a lock on the data
    pub fn is_running(&self) -> core::result::Result<bool, &'static str> {
        self.0.get().map(|g| g.running())
    }
}

#[pymethods]
#[doc(hidden)]
impl Op {
    #[getter]
    #[doc(hidden)]
    fn running(&self) -> PyResult<bool> {
        self.is_running().map_err(crate::make_err)
    }
    #[getter]
    #[doc(hidden)]
    fn gamepad(&self) -> PyResult<hardware::gamepad::Gamepad> {
        self.get_gamepad().map_err(crate::make_err)
    }
}

/// Sleeps for a certain amount of seconds
///
/// THIS BLOCKS THE CURRENT THREAD
#[pyfunction]
#[doc(hidden)]
fn sleep(seconds: f64) -> PyResult<()> {
    std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
    Ok(())
}

/// An Autonomous Annotation (Decorator) for Python
///
/// This annotation is used to mark a function as an autonomous function.
#[pyclass]
#[doc(hidden)]
struct Auto(String);
#[pymethods]
impl Auto {
    #[new]
    #[doc(hidden)]
    fn __new__(name: &str) -> Self {
        Self(name.to_string())
    }
    #[doc(hidden)]
    #[pyo3(signature = (*args, **_kwargs))]
    fn __call__(
        &self,
        _py: Python<'_>,
        args: &pyo3::types::PyTuple,
        _kwargs: Option<&pyo3::types::PyDict>,
    ) -> PyResult<Py<PyAny>> {
        let func = args.get_item(0)?;
        func.extract::<Py<PyAny>>()
    }
}

/// A Teleop Annotation (Decorator) for Python
///
/// This annotation is used to mark a function as a teleop function.
#[pyclass]
#[doc(hidden)]
struct Teleop(String);
#[pymethods]
impl Teleop {
    #[new]
    #[doc(hidden)]
    fn __new__(name: &str) -> Self {
        Self(name.to_string())
    }
    #[doc(hidden)]
    #[pyo3(signature = (*args, **_kwargs))]
    fn __call__(
        &self,
        _py: Python<'_>,
        args: &pyo3::types::PyTuple,
        _kwargs: Option<&pyo3::types::PyDict>,
    ) -> PyResult<Py<PyAny>> {
        let func = args.get_item(0)?;
        func.extract::<Py<PyAny>>()
    }
}

/// Constructs the Python module
///
/// This function is called by the Python interpreter when the module is imported.
#[pymodule]
#[doc(hidden)]
pub fn arc(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    m.add_class::<Teleop>()?;
    m.add_class::<Auto>()?;
    m.add_class::<Op>()?;
    m.add("OK", true)?;

    // Submodules
    //
    // Currently there is no better way to do this.
    // See https://github.com/DranikiRobotics/arc/issues/3
    crate::pymod!(hardware -> hardware::hardware_submodule, "arc.hardware", py, m);
    crate::pymod!(math -> math::math_submodule, "arc.math", py, m);

    Ok(())
}
