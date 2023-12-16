use crate::threadsafe::{self, ThreadSafe};
use pyo3::prelude::*;

/// Hardware submodule
#[path = "hardware/__init__.rs"]
pub mod _hardware;

#[doc(hidden)]
fn make_err(e: &'static str) -> PyErr {
    PyErr::new::<pyo3::exceptions::PyIOError, _>(e)
}

/// The struct that actually contains the necessary data for the op mode
/// to run.
/// 
/// This struct should only be used for mutating the data outside of the 
/// op mode thread. For reading the up to date data, use the `Op` struct.
#[derive(Debug)]
pub struct OpHolder {
    running: threadsafe::ThreadSafeBool,
    gamepad: _hardware::gamepad::Gamepad,
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
    pub fn gamepad(&self) -> &_hardware::gamepad::Gamepad {
        &self.gamepad
    }
    /// Stops the op mode
    /// 
    /// DO NOT CALL THIS FROM THE OP MODE THREAD
    pub fn stop(&self) {
        self.running.get_mut().unwrap().set(false);
    }
}

unsafe impl Send for OpHolder {}
unsafe impl Sync for OpHolder {}

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
/// ```rust,no_run,ignore
/// let op = pylib::Op::new(gamepad_wrapper);
/// let op_wrapper = pylib::Op::wrap(&op);
/// 
/// // IO Thread
/// op.get_mut()?
/// ```
#[pyclass]
#[derive(Debug, Clone)]
pub struct Op(ThreadSafe<OpHolder>);

impl Op {
    /// This creates a new `ThreadSafe<OpHolder>` struct. NOT a `Op` struct.
    /// 
    /// You then need to wrap it in a `Op` struct using the [`Op::wrap()`] method.
    pub fn new(gamepad: _hardware::gamepad::Gamepad) -> ThreadSafe<OpHolder> {
        ThreadSafe::new(OpHolder {
            running: true.into(),
            gamepad,
        })
    }
    /// Wraps a `ThreadSafe<OpHolder>` in a `Op` struct.
    pub fn wrap(op: &ThreadSafe<OpHolder>) -> Self {
        Self(op.clone())
    }
    /// Returns a new [`Gamepad`] struct that is a clone of the one in the op mode.
    /// 
    /// (It's an `Arc` so it's cheap to clone)
    /// 
    /// [`Gamepad`]: _hardware/gamepad/struct.Gamepad.html
    pub fn get_gamepad(&self) -> threadsafe::TSResult<_hardware::gamepad::Gamepad> {
        self.0.get().map(|g| g.gamepad().clone())
    }
    /// Returns whether the op mode is running
    /// 
    /// This call aquires a lock on the data
    pub fn is_running(&self) -> threadsafe::TSResult<bool> {
        self.0.get().map(|g| g.running())
    }
    
}

#[pymethods]
#[doc(hidden)]
impl Op {
    #[getter]
    #[doc(hidden)]
    fn running(&self) -> PyResult<bool> {
        self.is_running().map_err(make_err)
    }
    #[getter]
    #[doc(hidden)]
    fn gamepad(&self) -> PyResult<_hardware::gamepad::Gamepad> {
        self.get_gamepad().map_err(make_err)
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
struct Auto(Py<PyAny>);
#[pymethods]
impl Auto {
    #[new]
    #[doc(hidden)]
    fn __new__(wraps: Py<PyAny>) -> Self {
        Self(wraps)
    }
    #[doc(hidden)]
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &pyo3::types::PyTuple,
        kwargs: Option<&pyo3::types::PyDict>,
    ) -> PyResult<Py<PyAny>> {
        self.0.call(py, args, kwargs)
    }
}

/// A Teleop Annotation (Decorator) for Python
/// 
/// This annotation is used to mark a function as a teleop function.
#[pyclass]
#[doc(hidden)]
struct Teleop(Py<PyAny>);
#[pymethods]
impl Teleop {
    #[new]
    #[doc(hidden)]
    fn __new__(wraps: Py<PyAny>) -> Self {
        Self(wraps)
    }
    #[doc(hidden)]
    #[pyo3(signature = (*args, **kwargs))]
    fn __call__(
        &self,
        py: Python<'_>,
        args: &pyo3::types::PyTuple,
        kwargs: Option<&pyo3::types::PyDict>,
    ) -> PyResult<Py<PyAny>> {
        self.0.call(py, args, kwargs)
    }
}

/// Constructs the Python module
/// 
/// This function is called by the Python interpreter when the module is imported.
#[pymodule]
#[doc(hidden)]
pub fn arc(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sleep, m)?)?;
    m.add_class::<Teleop>()?;
    m.add_class::<Auto>()?;
    m.add_class::<Op>()?;
    m.add("OK", true)?;

    // Modules
    m.add_wrapped(pyo3::wrap_pymodule!(_hardware::hardware))?;

    Ok(())
}
