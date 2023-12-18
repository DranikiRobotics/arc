//! Implementations of the `Gamepad` trait for various gamepads.

use super::*;

/// A gamepad with no inputs.
#[repr(C)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct LogitechF310 {
    // Dpad
    dpad_up: bool,
    dpad_down: bool,
    dpad_left: bool,
    dpad_right: bool,

    // Sticks
    left_stick_x: f64,
    left_stick_y: f64,
    right_stick_x: f64,
    right_stick_y: f64,

    // Triggers
    left_trigger: f64,
    right_trigger: f64,

    // Buttons
    a: bool,
    b: bool,
    x: bool,
    y: bool,

    // Bumpers
    left_bumper: bool,
    right_bumper: bool,

    // Other
    back: bool,
    start: bool,
    left_stick_button: bool,
    right_stick_button: bool,
}

impl LogitechF310 {
    /// Creates a new `LogitechF310` with all values set to their defaults.
    #[inline]
    #[must_use = "This returns a new LogitechF310"]
    pub const fn new() -> Self {
        Self {
            dpad_up: false,
            dpad_down: false,
            dpad_left: false,
            dpad_right: false,

            left_stick_x: 0.0,
            left_stick_y: 0.0,
            right_stick_x: 0.0,
            right_stick_y: 0.0,

            left_trigger: 0.0,
            right_trigger: 0.0,

            a: false,
            b: false,
            x: false,
            y: false,

            left_bumper: false,
            right_bumper: false,

            back: false,
            start: false,
            left_stick_button: false,
            right_stick_button: false,
        }
    }
}

impl Gamepad for LogitechF310 {
    #[inline]
    fn dpad(&self) -> crate::Result<GamepadDpad> {
        Ok(GamepadDpad::new(
            self.dpad_up,
            self.dpad_down,
            self.dpad_left,
            self.dpad_right,
        ))
    }
    #[inline]
    fn left_stick(&self) -> crate::Result<GamepadStick> {
        Ok(GamepadStick::new(
            self.left_stick_x,
            self.left_stick_y,
            self.left_stick_button,
        ))
    }
    #[inline]
    fn right_stick(&self) -> crate::Result<GamepadStick> {
        Ok(GamepadStick::new(
            self.right_stick_x,
            self.right_stick_y,
            self.right_stick_button,
        ))
    }
    #[inline]
    fn left_trigger(&self) -> crate::Result<f64> {
        Ok(self.left_trigger)
    }
    #[inline]
    fn right_trigger(&self) -> crate::Result<f64> {
        Ok(self.right_trigger)
    }
    #[inline]
    fn a(&self) -> crate::Result<bool> {
        Ok(self.a)
    }
    #[inline]
    fn b(&self) -> crate::Result<bool> {
        Ok(self.b)
    }
    #[inline]
    fn x(&self) -> crate::Result<bool> {
        Ok(self.x)
    }
    #[inline]
    fn y(&self) -> crate::Result<bool> {
        Ok(self.y)
    }
    #[inline]
    fn left_bumper(&self) -> crate::Result<bool> {
        Ok(self.left_bumper)
    }
    #[inline]
    fn right_bumper(&self) -> crate::Result<bool> {
        Ok(self.right_bumper)
    }
    #[inline]
    fn back(&self) -> crate::Result<bool> {
        Ok(self.back)
    }
    #[inline]
    fn start(&self) -> crate::Result<bool> {
        Ok(self.start)
    }
}

impl MutableGamepad for LogitechF310 {
    #[inline]
    fn set_dpad(&mut self, dpad: GamepadDpad) -> crate::Result {
        self.dpad_up = dpad.up;
        self.dpad_down = dpad.down;
        self.dpad_left = dpad.left;
        self.dpad_right = dpad.right;
        Ok(())
    }
    #[inline]
    fn set_left_stick(&mut self, stick: GamepadStick) -> crate::Result {
        self.left_stick_x = stick.x;
        self.left_stick_y = stick.y;
        self.left_stick_button = stick.pressed;
        Ok(())
    }
    #[inline]
    fn set_right_stick(&mut self, stick: GamepadStick) -> crate::Result {
        self.right_stick_x = stick.x;
        self.right_stick_y = stick.y;
        self.right_stick_button = stick.pressed;
        Ok(())
    }
    #[inline]
    fn set_left_trigger(&mut self, trigger: f64) -> crate::Result {
        self.left_trigger = trigger;
        Ok(())
    }
    #[inline]
    fn set_right_trigger(&mut self, trigger: f64) -> crate::Result {
        self.right_trigger = trigger;
        Ok(())
    }
    #[inline]
    fn set_a(&mut self, value: bool) -> crate::Result {
        self.a = value;
        Ok(())
    }
    #[inline]
    fn set_b(&mut self, value: bool) -> crate::Result {
        self.b = value;
        Ok(())
    }
    #[inline]
    fn set_x(&mut self, value: bool) -> crate::Result {
        self.x = value;
        Ok(())
    }
    #[inline]
    fn set_y(&mut self, value: bool) -> crate::Result {
        self.y = value;
        Ok(())
    }
    #[inline]
    fn set_left_bumper(&mut self, value: bool) -> crate::Result {
        self.left_bumper = value;
        Ok(())
    }
    #[inline]
    fn set_right_bumper(&mut self, value: bool) -> crate::Result {
        self.right_bumper = value;
        Ok(())
    }
    #[inline]
    fn set_back(&mut self, value: bool) -> crate::Result {
        self.back = value;
        Ok(())
    }
    #[inline]
    fn set_start(&mut self, value: bool) -> crate::Result {
        self.start = value;
        Ok(())
    }
}
