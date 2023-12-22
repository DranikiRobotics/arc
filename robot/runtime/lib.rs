#![doc = include_str!("../README.md")]

#![warn(missing_docs, unused, clippy::all, unsafe_code)]
#![deny(missing_debug_implementations)]

use pyo3::prelude::*;

#[derive(Debug, Clone)]
struct RuntimeOpMode {
    hardware: hardware::HardwareMap,
    name: String
}

impl RuntimeOpMode {
    fn start(&mut self, hardware: &mut hardware::HardwareMap) -> Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    OpNotFound,
}

pub type Result<T = (), E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct PyRuntime {
    ops: Vec<RuntimeOpMode>,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl PyRuntime {
    pub fn init() -> Self {
        Self {
            ops: Vec::new(),
            thread: None,
        }
    }
    pub fn start<Name: ToString>(&mut self, name: Name) -> Result<()> {
        let name = name.to_string();
        let op = self.ops.iter_mut()
            .find(|op| op.name == name)
            .ok_or(Error::OpNotFound)?;
        op.start(&mut self.hardware)
    }
}

// fn main() -> PyResult<()> {
    // Setup hardware components
    // let (gamepad, gamepad_wrapper) = pylib::setup_wrapped_component!(
    //     pylib::arc_robot_hardware::gamepad::impls::LogitechF310::default();
    //     pylib::__init__::hardware::gamepad::Gamepad
    // );
    // let (op, op_wrapper) = pylib::setup_wrapped_component!(gamepad_wrapper; pylib::__init__::Op);

    // IO Thread
//     std::thread::spawn(move || {
//         use hardware::gamepad::MutableGamepad as _;
// 
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         gamepad.get_mut()?.set_a(true)?;
// 
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         gamepad.get_mut()?.set_a(false)?;
// 
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         gamepad.get_mut()?.set_a(true)?;
// 
//         std::thread::sleep(std::time::Duration::from_secs(1));
//         op.get_mut()?.stop()?;
// 
//         op.get_mut()?.running_time();
// 
//         IO_OK
//     });

    // Main Thread
//     let mut code = String::new();
//     std::fs::File::open("./examples/auto.py")?.read_to_string(&mut code)?;
// 
//     // Python (Main Thread)
//     Python::with_gil(move |py| {
//         let main_func: pyo3::Py<pyo3::PyAny> = PyModule::from_code(py, &code, "auto.py", "")?
//             .getattr("main")?
//             .into();
//         main_func.call1(py, (op_wrapper,))?;
//         Ok(())
//     })
// }
