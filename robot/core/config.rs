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

/// A trait for common functionality that can be used to configure the robot
pub trait RobotConfig {
    /// This function is called before the python interpreter is initialized
    /// 
    /// This is useful for loading rust libraries that will then be used by python.
    /// In fact, this is exactly how ARC is loaded.
    fn python_preload() {}
    type Args: IntoPy<Py<PyTuple>> + Default;
    fn build_python_main_function_args<'a>(_py: &pyo3::Python<'_>, _io: &crate::io::IO) -> (Self::Args, Option<&'a PyDict>) {
        (Self::Args::default(), None)
    }
}
