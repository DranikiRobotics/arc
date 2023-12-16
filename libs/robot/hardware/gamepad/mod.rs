use libtrig::{Angle2D, Vec2D};

pub mod impls;

pub trait Gamepad: core::fmt::Debug {
    /// Returns the state of the dpad
    ///
    /// Includes up, down, left, and right
    fn dpad(&self) -> GamepadDpad;
    /// Returns the state of the left stick
    ///
    /// Includes x, y, and pressed
    fn left_stick(&self) -> GamepadStick;
    /// Returns the state of the right stick
    ///
    /// Includes x, y, and pressed
    fn right_stick(&self) -> GamepadStick;
    /// Returns the state of the left trigger
    fn left_trigger(&self) -> f64;
    /// Returns the state of the right trigger
    fn right_trigger(&self) -> f64;
    /// Is the A button pressed?
    fn a(&self) -> bool;
    /// Is the B button pressed?
    fn b(&self) -> bool;
    /// Is the X button pressed?
    fn x(&self) -> bool;
    /// Is the Y button pressed?
    fn y(&self) -> bool;
    /// Is the left bumper pressed?
    fn left_bumper(&self) -> bool;
    /// Is the right bumper pressed?
    fn right_bumper(&self) -> bool;

    /// A non-standard 'back' button
    #[inline]
    fn back(&self) -> bool {
        false
    }
    /// A non-standard 'start' button
    #[inline]
    fn start(&self) -> bool {
        false
    }
}

/// Allows for the gamepad to be modified
pub trait MutableGamepad: Gamepad {
    fn set_dpap(&mut self, dpad: GamepadDpad);
    fn set_left_stick(&mut self, stick: GamepadStick);
    fn set_right_stick(&mut self, stick: GamepadStick);
    fn set_left_trigger(&mut self, trigger: f64);
    fn set_right_trigger(&mut self, trigger: f64);
    fn set_a(&mut self, value: bool);
    fn set_b(&mut self, value: bool);
    fn set_x(&mut self, value: bool);
    fn set_y(&mut self, value: bool);
    fn set_left_bumper(&mut self, value: bool);
    fn set_right_bumper(&mut self, value: bool);
    fn set_back(&mut self, _value: bool) {}
    fn set_start(&mut self, _value: bool) {}
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct GamepadStick {
    pub x: f64,
    pub y: f64,
    pub pressed: bool,
}

impl GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    pub const fn new(x: f64, y: f64, pressed: bool) -> Self {
        Self { x, y, pressed }
    }
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    pub const fn default() -> Self {
        Self::new(0.0, 0.0, false)
    }
    #[inline]
    pub fn angle(&self) -> libtrig::Angle2D {
        libtrig::prelude!();
        libtrig::Angle2D::from_radians(self.y.atan2(self.x))
    }
}

impl From<Angle2D> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from(angle: Angle2D) -> Self {
        libtrig::prelude!();
        Self::new(angle.cos(), angle.sin(), false)
    }
}

impl From<GamepadStick> for Angle2D {
    #[inline]
    #[must_use = "This returns a new Angle2D"]
    fn from(stick: GamepadStick) -> Self {
        stick.angle()
    }
}

impl From<Vec2D> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from(vec: Vec2D) -> Self {
        Self::new(vec.x(), vec.y(), false)
    }
}

impl From<GamepadStick> for Vec2D {
    #[inline]
    #[must_use = "This returns a new Vec2D"]
    fn from(stick: GamepadStick) -> Self {
        Self::new(stick.x, stick.y)
    }
}

impl From<(f64, f64, bool)> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from((x, y, pressed): (f64, f64, bool)) -> Self {
        Self::new(x, y, pressed)
    }
}

impl From<(f64, f64)> for GamepadStick {
    #[inline]
    #[must_use = "This returns a new GamepadStick"]
    fn from((x, y): (f64, f64)) -> Self {
        Self::new(x, y, false)
    }
}

impl Eq for GamepadStick {}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadDpad {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl GamepadDpad {
    #[inline]
    #[must_use = "This returns a new GamepadDpad"]
    pub const fn new(up: bool, down: bool, left: bool, right: bool) -> Self {
        Self {
            up,
            down,
            left,
            right,
        }
    }
    #[inline]
    #[must_use = "This returns a new GamepadDpad"]
    pub const fn default() -> Self {
        Self::new(false, false, false, false)
    }
}

impl From<(bool, bool, bool, bool)> for GamepadDpad {
    #[inline]
    #[must_use = "This returns a new GamepadDpad"]
    fn from((up, down, left, right): (bool, bool, bool, bool)) -> Self {
        Self::new(up, down, left, right)
    }
}

impl From<GamepadDpad> for (bool, bool, bool, bool) {
    #[inline]
    #[must_use = "This returns a new tuple"]
    fn from(dpad: GamepadDpad) -> Self {
        (dpad.up, dpad.down, dpad.left, dpad.right)
    }
}
