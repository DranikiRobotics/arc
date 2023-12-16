use pylib::arc as arc_py_module;
use pyo3::prelude::*;
use std::io::Read;

fn main() -> PyResult<()> {
    pyo3::append_to_inittab!(arc_py_module);

    pyo3::prepare_freethreaded_python();

    let mut code = String::new();
    std::fs::File::open("./examples/auto.py")?.read_to_string(&mut code)?;

    Python::with_gil(|py| {
        let fun: Py<PyAny> = PyModule::from_code(py, &code, "main.py", "main")?
            .getattr("main")?
            .into();

        let gamepad = pylib::_hardware::gamepad::Gamepad::new(
            pylib::hardware::gamepad::impls::LogitechF310::default(),
        );
        let gamepad_wrapper = pylib::_hardware::gamepad::Gamepad::wrap(&gamepad);
        let op = pylib::Op::new(gamepad_wrapper);
        let op_wrapper = pylib::Op::wrap(&op);

        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(1));
            gamepad.get_mut().unwrap().set_a(true);
            std::thread::sleep(std::time::Duration::from_secs(1));
            gamepad.get_mut().unwrap().set_a(false);
            std::thread::sleep(std::time::Duration::from_secs(1));
            gamepad.get_mut().unwrap().set_a(true);
            std::thread::sleep(std::time::Duration::from_secs(1));
            op.get_mut().unwrap().stop();
        });

        fun.call1(py, (op_wrapper,))?;
        Ok(())
    })
}
