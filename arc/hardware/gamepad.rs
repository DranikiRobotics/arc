use hardware::gamepad::MutableGamepad;
use crate::threadsafe::ThreadSafe;
use pyo3::prelude::*;

#[derive(Debug)]
pub struct GamepadHolder {
    gamepad: Box<dyn MutableGamepad>,
}

impl core::ops::Deref for GamepadHolder {
    type Target = dyn MutableGamepad;
    fn deref(&self) -> &Self::Target {
        &*self.gamepad
    }
}

impl core::ops::DerefMut for GamepadHolder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.gamepad
    }
}

unsafe impl Send for GamepadHolder {}
unsafe impl Sync for GamepadHolder {}

#[pyclass]
#[derive(Debug)]
pub struct Gamepad(ThreadSafe<GamepadHolder>);

impl Gamepad {
    pub fn new<G: MutableGamepad + 'static>(gamepad: G) -> ThreadSafe<GamepadHolder> {
        ThreadSafe::new(GamepadHolder {
            gamepad: Box::new(gamepad),
        })
    }
    pub fn wrap(gamepad: &ThreadSafe<GamepadHolder>) -> Self {
        Self(gamepad.clone())
    }
    fn make_err(e: &'static str) -> PyErr {
        PyErr::new::<pyo3::exceptions::PyIOError, _>(e)
    }
}

impl Clone for Gamepad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[pymethods]
impl Gamepad {
    #[getter]
    pub fn x(&self) -> PyResult<bool> {
        self.0.get().map(|g| g.x()).map_err(Self::make_err)
    }
    #[getter]
    pub fn y(&self) -> PyResult<bool> {
        self.0.get().map(|g| g.y()).map_err(Self::make_err)
    }
    #[getter]
    pub fn a(&self) -> PyResult<bool> {
        self.0.get().map(|g| g.a()).map_err(Self::make_err)
    }
    #[getter]
    pub fn b(&self) -> PyResult<bool> {
        self.0.get().map(|g| g.b()).map_err(Self::make_err)
    }
}

#[pymodule]
pub fn gamepad(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Gamepad>()?;

    // todo!("add GamepadStick and GamepadDpad");

    eprintln!("GamepadStick and GamepadDpad are not implemented yet");

    Ok(())
}
