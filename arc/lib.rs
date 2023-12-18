#![doc = include_str!("./README.md")]
#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

#[doc(hidden)]
pub use arc_robot_hardware;

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

// pub fn run_op() -> pyo3::PyResult<()> {
//
// }
