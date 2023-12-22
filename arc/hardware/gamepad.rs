//! Gamepad module for the arc crate
//!
//! Python identifier: `arc.hardware.gamepad`

use crate::threadsafe::ThreadSafe;
use arc_robot_hardware::gamepad::{self as gp, Gamepad as _};
use pyo3::prelude::*;

/// The struct that actually contains the necessary data for the gamepad
/// to function.
///
/// This struct should only be used for mutating the data outside of the
/// gamepad thread. For reading the up to date data, use the `Gamepad` struct.
#[derive(Debug)]
pub struct GamepadHolder {
    gamepad: Box<dyn gp::MutableGamepad>,
}

impl Default for GamepadHolder {
    fn default() -> Self {
        Self {
            gamepad: Box::new(gp::impls::Stub),
        }
    }
}

macro_rules! i {
    (g $n: ident $ty:ty) => {
        #[inline]
        fn $n(&self) -> $crate::arc_robot_hardware::Result<$ty> {
            self.gamepad.$n()
        }
    };
    (s $n: ident $ty:ty) => {
        #[inline]
        fn $n(&mut self, value: $ty) -> $crate::arc_robot_hardware::Result {
            self.gamepad.$n(value)
        }
    };
}

impl gp::Gamepad for GamepadHolder {
    i!(g a bool);
    i!(g b bool);
    i!(g x bool);
    i!(g y bool);
    i!(g left_bumper bool);
    i!(g right_bumper bool);
    i!(g back bool);
    i!(g start bool);
    i!(g left_trigger f64);
    i!(g right_trigger f64);
    i!(g left_stick gp::GamepadStick);
    i!(g right_stick gp::GamepadStick);
    i!(g dpad gp::GamepadDpad);
}

impl gp::MutableGamepad for GamepadHolder {
    i!(s set_a bool);
    i!(s set_b bool);
    i!(s set_x bool);
    i!(s set_y bool);
    i!(s set_left_bumper bool);
    i!(s set_right_bumper bool);
    i!(s set_back bool);
    i!(s set_start bool);
    i!(s set_left_trigger f64);
    i!(s set_right_trigger f64);
    i!(s set_left_stick gp::GamepadStick);
    i!(s set_right_stick gp::GamepadStick);
    i!(s set_dpad gp::GamepadDpad);
}

crate::threadsafe::thread_safe!(GamepadHolder);

/// A struct that holds the state of a gamepad stick
#[pyclass]
#[derive(Debug, Clone)]
pub struct GamepadStick(gp::GamepadStick);

impl GamepadStick {
    /// Returns the x value of the stick.
    pub fn get_x(&self) -> f64 {
        self.0.x
    }
    /// Returns the y value of the stick.
    pub fn get_y(&self) -> f64 {
        self.0.y
    }
    /// Returns whether or not the stick is pressed.
    pub fn get_pressed(&self) -> bool {
        self.0.pressed
    }
    /// Converts the stick into an angle.
    pub fn into_angle(&self) -> libtrig::Angle2D {
        libtrig::Angle2D::from((self.get_x(), self.get_y()))
    }
    /// Converts the stick into a vector.
    pub fn into_vector(&self) -> libtrig::Vec2D {
        libtrig::Vec2D::from((self.get_x(), self.get_y()))
    }
}

impl From<GamepadStick> for libtrig::Angle2D {
    fn from(stick: GamepadStick) -> Self {
        stick.into_angle()
    }
}

impl From<libtrig::Angle2D> for GamepadStick {
    fn from(angle: libtrig::Angle2D) -> Self {
        libtrig::prelude!();
        Self(gp::GamepadStick {
            x: angle.cos(),
            y: angle.sin(),
            pressed: false,
        })
    }
}

impl From<GamepadStick> for libtrig::Vec2D {
    fn from(stick: GamepadStick) -> Self {
        stick.into_vector()
    }
}

impl From<libtrig::Vec2D> for GamepadStick {
    fn from(vector: libtrig::Vec2D) -> Self {
        let vector = vector.normalize();
        Self(gp::GamepadStick {
            x: vector.x(),
            y: vector.y(),
            pressed: false,
        })
    }
}

#[pymethods]
impl GamepadStick {
    #[getter]
    fn x(&self) -> PyResult<f64> {
        Ok(self.get_x())
    }
    #[getter]
    fn y(&self) -> PyResult<f64> {
        Ok(self.get_y())
    }
    #[getter]
    fn pressed(&self) -> PyResult<bool> {
        Ok(self.get_pressed())
    }
    fn as_angle(&self) -> PyResult<crate::__init__::math::angle::Angle> {
        Ok(self.into_angle().into())
    }
    fn as_vec2d(&self) -> PyResult<crate::__init__::math::vec2d::Vec2D> {
        Ok(self.into_vector().into())
    }
}

/// A struct that holds the state of a gamepad dpad
#[pyclass]
#[derive(Debug, Clone)]
pub struct GamepadDpad(gp::GamepadDpad);

impl From<gp::GamepadDpad> for GamepadDpad {
    fn from(dpad: gp::GamepadDpad) -> Self {
        Self(dpad)
    }
}

impl From<GamepadDpad> for gp::GamepadDpad {
    fn from(dpad: GamepadDpad) -> Self {
        dpad.0
    }
}

#[pymethods]
impl GamepadDpad {
    #[getter]
    fn up(&self) -> PyResult<bool> {
        Ok(self.0.up)
    }
    #[getter]
    fn down(&self) -> PyResult<bool> {
        Ok(self.0.down)
    }
    #[getter]
    fn left(&self) -> PyResult<bool> {
        Ok(self.0.left)
    }
    #[getter]
    fn right(&self) -> PyResult<bool> {
        Ok(self.0.right)
    }
}

/// The struct that is used to access the gamepad data from python.
///
/// This struct is thread safe, and can be used to read the gamepad data
/// from any thread.
///
/// This struct should not be used to modify the gamepad data. For that,
/// use the `GamepadHolder` struct.
#[pyclass]
#[derive(Default, Debug, Clone)]
pub struct Gamepad(ThreadSafe<GamepadHolder>);

impl<G: gp::MutableGamepad + 'static> crate::PyWrappedComponent<G> for Gamepad {
    type Holder = GamepadHolder;
    fn new(gamepad: G) -> ThreadSafe<Self::Holder> {
        ThreadSafe::new(GamepadHolder {
            gamepad: Box::new(gamepad),
        })
    }
    fn wrap(gamepad: &ThreadSafe<Self::Holder>) -> Self {
        Self(gamepad.clone())
    }
}

impl Gamepad {
    /// This creates a new `ThreadSafe<GamepadHolder>` struct. NOT a `Gamepad` struct.
    ///
    /// You then need to wrap it in a `Gamepad` struct using the [`Gamepad::wrap()`] method.
    pub fn new<G: gp::MutableGamepad + 'static>(gamepad: G) -> ThreadSafe<GamepadHolder> {
        ThreadSafe::new(GamepadHolder {
            gamepad: Box::new(gamepad),
        })
    }
    /// Wraps a `ThreadSafe<GamepadHolder>` in a `Gamepad` struct.
    pub fn wrap(gamepad: &ThreadSafe<GamepadHolder>) -> Self {
        Self(gamepad.clone())
    }
    /// Returns the state of the dpad
    ///
    /// Includes up, down, left, and right
    pub fn get_dpad(&self) -> core::result::Result<GamepadDpad, &'static str> {
        self.0
            .get()?
            .dpad()
            .map(|d| GamepadDpad(d))
            .map_err(|e| e.into())
    }
    /// Returns the state of the left stick
    ///
    /// Includes x, y, and pressed
    pub fn get_left_stick(&self) -> core::result::Result<GamepadStick, &'static str> {
        self.0
            .get()?
            .left_stick()
            .map(|d| GamepadStick(d))
            .map_err(|e| e.into())
    }
    /// Returns the state of the right stick
    ///
    /// Includes x, y, and pressed
    pub fn get_right_stick(&self) -> core::result::Result<GamepadStick, &'static str> {
        self.0
            .get()?
            .right_stick()
            .map(|d| GamepadStick(d))
            .map_err(|e| e.into())
    }
    /// Returns the state of the left trigger
    pub fn get_left_trigger(&self) -> core::result::Result<f64, &'static str> {
        self.0.get()?.left_trigger().map_err(|e| e.into())
    }
    /// Returns the state of the right trigger
    pub fn get_right_trigger(&self) -> core::result::Result<f64, &'static str> {
        self.0.get()?.right_trigger().map_err(|e| e.into())
    }
    /// Is the 'x' button pressed?
    pub fn get_x(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.x().map_err(|e| e.into())
    }
    /// Is the 'y' button pressed?
    pub fn get_y(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.y().map_err(|e| e.into())
    }
    /// Is the 'a' button pressed?
    pub fn get_a(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.a().map_err(|e| e.into())
    }
    /// Is the 'b' button pressed?
    pub fn get_b(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.b().map_err(|e| e.into())
    }
    /// Is the left bumper pressed?
    pub fn get_left_bumper(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.left_bumper().map_err(|e| e.into())
    }
    /// Is the right bumper pressed?
    pub fn get_right_bumper(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.right_bumper().map_err(|e| e.into())
    }

    /// A non-standard 'back' button
    pub fn get_back(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.back().map_err(|e| e.into())
    }
    /// A non-standard 'start' button
    pub fn get_start(&self) -> core::result::Result<bool, &'static str> {
        self.0.get()?.start().map_err(|e| e.into())
    }
}

#[pymethods]
impl Gamepad {
    #[getter]
    fn dpad(&self) -> PyResult<GamepadDpad> {
        self.get_dpad().map_err(crate::make_err)
    }
    #[getter]
    fn left_stick(&self) -> PyResult<GamepadStick> {
        self.get_left_stick().map_err(crate::make_err)
    }
    #[getter]
    fn right_stick(&self) -> PyResult<GamepadStick> {
        self.get_right_stick().map_err(crate::make_err)
    }
    #[getter]
    fn left_trigger(&self) -> PyResult<f64> {
        self.get_left_trigger().map_err(crate::make_err)
    }
    #[getter]
    fn right_trigger(&self) -> PyResult<f64> {
        self.get_right_trigger().map_err(crate::make_err)
    }
    #[getter]
    fn x(&self) -> PyResult<bool> {
        self.get_x().map_err(crate::make_err)
    }
    #[getter]
    fn y(&self) -> PyResult<bool> {
        self.get_y().map_err(crate::make_err)
    }
    #[getter]
    fn a(&self) -> PyResult<bool> {
        self.get_a().map_err(crate::make_err)
    }
    #[getter]
    fn b(&self) -> PyResult<bool> {
        self.get_b().map_err(crate::make_err)
    }
    #[getter]
    fn left_bumper(&self) -> PyResult<bool> {
        self.get_left_bumper().map_err(crate::make_err)
    }
    #[getter]
    fn right_bumper(&self) -> PyResult<bool> {
        self.get_right_bumper().map_err(crate::make_err)
    }

    #[getter]
    fn back(&self) -> PyResult<bool> {
        self.get_back().map_err(crate::make_err)
    }
    #[getter]
    fn start(&self) -> PyResult<bool> {
        self.get_start().map_err(crate::make_err)
    }
}

/// The gamepad module
#[doc(hidden)]
pub(crate) fn gamepad_submodule(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Gamepad>()?;
    m.add_class::<GamepadStick>()?;
    m.add_class::<GamepadDpad>()?;

    Ok(())
}
