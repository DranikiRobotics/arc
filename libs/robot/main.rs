use pylib::__init__::arc as arc_py_module;
use pylib::arc_robot_hardware::IO_OK;
use pyo3::prelude::*;
use std::io::Read;

fn main() -> PyResult<()> {
    // Initialize Python
    pyo3::append_to_inittab!(arc_py_module);
    pyo3::prepare_freethreaded_python();

    // Setup hardware components
    let (gamepad, gamepad_wrapper) = pylib::setup_wrapped_component!(
        pylib::arc_robot_hardware::gamepad::impls::LogitechF310::default();
        pylib::__init__::hardware::gamepad::Gamepad
    );
    let (op, op_wrapper) = pylib::setup_wrapped_component!(gamepad_wrapper; pylib::__init__::Op);

    // IO Thread
    std::thread::spawn(move || {
        use hardware::gamepad::MutableGamepad as _;

        std::thread::sleep(std::time::Duration::from_secs(1));
        gamepad.get_mut()?.set_a(true)?;

        std::thread::sleep(std::time::Duration::from_secs(1));
        gamepad.get_mut()?.set_a(false)?;

        std::thread::sleep(std::time::Duration::from_secs(1));
        gamepad.get_mut()?.set_a(true)?;

        std::thread::sleep(std::time::Duration::from_secs(1));
        op.get_mut()?.stop()?;

        op.get_mut()?.running_time();

        IO_OK
    });

    // Main Thread
    let mut code = String::new();
    std::fs::File::open("./examples/auto.py")?.read_to_string(&mut code)?;

    // Python (Main Thread)
    Python::with_gil(move |py| {
        let main_func: pyo3::Py<pyo3::PyAny> = PyModule::from_code(py, &code, "auto.py", "")?
            .getattr("main")?
            .into();
        main_func.call1(py, (op_wrapper,))?;
        Ok(())
    })
}
