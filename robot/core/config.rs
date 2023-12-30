//! This module contains the [RobotConfig] trait and the [DefaultRobotConfig] struct
//! 
//! This is primarily used to configure the robot at a higher level than just changing the
//! entire codebase.
//! 
//! This is useful for things like loading rust libraries before the python
//! interpreter is initialized (which is exactly how ARC is loaded). Or for customizing
//! things like the logging system. Or for anything else that needs to be done before
//! the actual robot code is run.

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use crate::prelude::*;
use crate::internals::impls;
use impls::HardwareMapImpl as BuiltInHardwareMapImpl;
use impls::TelemetryImpl as BuiltInTelemetryImpl;
use impls::GamepadImpl as BuiltInGamepadImpl;

/// A trait for common functionality that can be used to configure the robot
#[cfg(not(feature = "only_builtins"))]
pub trait RobotConfig<
    H = BuiltInHardwareMapImpl,
    T = BuiltInTelemetryImpl,
    G = BuiltInGamepadImpl
> where
    H: HardwareMap,
    T: Telemetry,
    G: Gamepad
{
    /// This function is called before the python interpreter is initialized
    /// 
    /// This is useful for loading rust libraries that will then be used by python.
    /// In fact, this is exactly how ARC is loaded.
    fn python_preload() {}
    /// The type that will be passed to the python main function
    type Args: IntoPy<Py<PyTuple>> + Default;
    /// This function is called to build the arguments that will be passed to the python main function
    fn build_python_main_function_args<'a>(
        _py: &pyo3::Python<'_>, _op: &crate::RuntimeOp<H, T, G>
    ) -> (Self::Args, Option<&'a PyDict>) {
        (Self::Args::default(), None)
    }
}

/// A trait for common functionality that can be used to configure the robot
#[cfg(feature = "only_builtins")]
pub trait RobotConfig {
    /// This function is called before the python interpreter is initialized
    /// 
    /// This is useful for loading rust libraries that will then be used by python.
    /// In fact, this is exactly how ARC is loaded.
    fn python_preload() {}
    /// The type that will be passed to the python main function
    type Args: IntoPy<Py<PyTuple>> + Default;
    /// This function is called to build the arguments that will be passed to the python main function
    fn build_python_main_function_args<'a>(
        _py: &pyo3::Python<'_>, _op: &crate::RuntimeOp
    ) -> (Self::Args, Option<&'a PyDict>) {
        (Self::Args::default(), None)
    }
}
