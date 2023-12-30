#![doc = include_str!("./README.md")]

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

const PY_OK: pyo3::PyResult<()> = Ok(());

use dranikcore::prelude::*;
use pyo3::prelude::*;
use pyo3::Python;

#[cfg(not(feature = "dranik-only-builtins"))]
pub async fn main<
    PyArgs,
    RoboConf,
    HardwareMapImpl,
    TelemetryImpl,
    GamepadImpl,
>(
    op: dranikcore::RuntimeOp<
        HardwareMapImpl,
        TelemetryImpl,
        GamepadImpl,
    >,
) where
    PyArgs: IntoPy<Py<pyo3::types::PyTuple>> + Default,
    RoboConf: RobotConfig<
        HardwareMapImpl,
        TelemetryImpl,
        GamepadImpl,
        Args = PyArgs
    >,
    HardwareMapImpl: HardwareMap + 'static,
    TelemetryImpl: Telemetry + 'static,
    GamepadImpl: Gamepad + 'static,
{
    RoboConf::python_preload();

    pyo3::prepare_freethreaded_python();

    dranikcore::deblock(move || Python::with_gil(|py| {
        // add files to sys.path
        let syspath = py.import("sys")?
            .getattr("path")?
            .downcast::<pyo3::types::PyList>()?;
        syspath.insert(0, "./examples")?;

        let args = RoboConf::build_python_main_function_args(&py, &op);

        // import main module
        let main = py.import("auto")?;
        let mainfunc = main.getattr("main")?;
        mainfunc.call(args.0, args.1)?;
        PY_OK
    }))
        .await
        .expect("Failed to run python main")
        .expect("Python main returned an error");
}

#[cfg(feature = "dranik-only-builtins")]
pub async fn main<
    PyArgs,
    RoboConf,
>(op: dranikcore::RuntimeOp) where
    PyArgs: IntoPy<Py<pyo3::types::PyTuple>> + Default,
    RoboConf: RobotConfig<Args = PyArgs>
{
    RoboConf::python_preload();

    pyo3::prepare_freethreaded_python();

    dranikcore::deblock(move || Python::with_gil(|py| {
        // add files to sys.path
        let syspath = py.import("sys")?
            .getattr("path")?
            .downcast::<pyo3::types::PyList>()?;
        syspath.insert(0, "./examples")?;

        let args = RoboConf::build_python_main_function_args(&py, &op);

        // import main module
        let main = py.import("auto")?;
        let mainfunc = main.getattr("main")?;
        mainfunc.call(args.0, args.1)?;
        PY_OK
    }))
        .await
        .expect("Failed to run python main")
        .expect("Python main returned an error");
}
