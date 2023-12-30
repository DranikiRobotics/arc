//! Gamepad module for the arc crate
//!
//! Python identifier: `arc.hardware.gamepad`

use dranikcore::{gamepad as gp, threadsafe::ThreadSafe};
use gp::Gamepad as _;
use pyo3::prelude::*;

/// A struct that holds the state of a gamepad stick
#[pyclass]
#[derive(Debug, Clone)]
pub struct GamepadStick(gp::GamepadStick);

impl GamepadStick {
    /// Returns the x value of the stick.
    #[inline(always)]
    pub fn get_x(&self) -> f64 {
        self.0.x
    }
    /// Returns the y value of the stick.
    #[inline(always)]
    pub fn get_y(&self) -> f64 {
        self.0.y
    }
    /// Returns whether or not the stick is pressed.
    #[inline(always)]
    pub fn get_pressed(&self) -> bool {
        self.0.pressed
    }
    /// Converts the stick into an angle.
    #[inline]
    #[must_use = "This returns a new angle"]
    pub fn into_angle(&self) -> libtrig::Angle2D {
        libtrig::Angle2D::from((self.get_x(), self.get_y()))
    }
    /// Converts the stick into a vector.
    #[inline]
    #[must_use = "This returns a new vector"]
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
    #[inline(always)]
    fn x(&self) -> PyResult<f64> {
        Ok(self.get_x())
    }
    #[getter]
    #[inline(always)]
    fn y(&self) -> PyResult<f64> {
        Ok(self.get_y())
    }
    #[getter]
    #[inline(always)]
    fn pressed(&self) -> PyResult<bool> {
        Ok(self.get_pressed())
    }
    #[inline(always)]
    fn as_angle(&self) -> PyResult<crate::__init__::math::angle::Angle> {
        Ok(self.into_angle().into())
    }
    #[inline(always)]
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

#[cfg(feature = "dranik-only-builtins")]
type GImpl = ThreadSafe<dranikcore::internals::builtins::BuiltInGamepadImpl>;

/// The struct that is used to access the gamepad data from python.
///
/// This struct is thread safe, and can be used to read the gamepad data
/// from any thread.
///
/// This struct should not be used to modify the gamepad data.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Gamepad(GImpl);

impl From<GImpl> for Gamepad {
    #[inline(always)]
    #[cfg(feature = "dranik-only-builtins")]
    fn from(gamepad: GImpl) -> Self {
        Self(gamepad)
    }
}

impl Gamepad {
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
    #[inline(always)]
    fn dpad(&self) -> PyResult<GamepadDpad> {
        self.get_dpad().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn left_stick(&self) -> PyResult<GamepadStick> {
        self.get_left_stick().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn right_stick(&self) -> PyResult<GamepadStick> {
        self.get_right_stick().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn left_trigger(&self) -> PyResult<f64> {
        self.get_left_trigger().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn right_trigger(&self) -> PyResult<f64> {
        self.get_right_trigger().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn x(&self) -> PyResult<bool> {
        self.get_x().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn y(&self) -> PyResult<bool> {
        self.get_y().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn a(&self) -> PyResult<bool> {
        self.get_a().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn b(&self) -> PyResult<bool> {
        self.get_b().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn left_bumper(&self) -> PyResult<bool> {
        self.get_left_bumper().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
    fn right_bumper(&self) -> PyResult<bool> {
        self.get_right_bumper().map_err(crate::make_err)
    }

    #[getter]
    #[inline(always)]
    fn back(&self) -> PyResult<bool> {
        self.get_back().map_err(crate::make_err)
    }
    #[getter]
    #[inline(always)]
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
