use super::*;

// #[repr(C)]
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

impl Gamepad for LogitechF310 {
    #[inline]
    fn dpad(&self) -> GamepadDpad {
        GamepadDpad::new(self.dpad_up, self.dpad_down, self.dpad_left, self.dpad_right)
    }
    #[inline]
    fn left_stick(&self) -> GamepadStick {
        GamepadStick::new(self.left_stick_x, self.left_stick_y, self.left_stick_button)
    }
    #[inline]
    fn right_stick(&self) -> GamepadStick {
        GamepadStick::new(self.right_stick_x, self.right_stick_y, self.right_stick_button)
    }
    #[inline]
    fn left_trigger(&self) -> f64 {
        self.left_trigger
    }
    #[inline]
    fn right_trigger(&self) -> f64 {
        self.right_trigger
    }
    #[inline]
    fn a(&self) -> bool {
        self.a
    }
    #[inline]
    fn b(&self) -> bool {
        self.b
    }
    #[inline]
    fn x(&self) -> bool {
        self.x
    }
    #[inline]
    fn y(&self) -> bool {
        self.y
    }
    #[inline]
    fn left_bumper(&self) -> bool {
        self.left_bumper
    }
    #[inline]
    fn right_bumper(&self) -> bool {
        self.right_bumper
    }
    #[inline]
    fn back(&self) -> bool {
        self.back
    }
    #[inline]
    fn start(&self) -> bool {
        self.start
    }
}

impl MutableGamepad for LogitechF310 {
    #[inline]
    fn set_dpap(&mut self, dpad: GamepadDpad) {
        self.dpad_up = dpad.up;
        self.dpad_down = dpad.down;
        self.dpad_left = dpad.left;
        self.dpad_right = dpad.right;
    }
    #[inline]
    fn set_left_stick(&mut self, stick: GamepadStick) {
        self.left_stick_x = stick.x;
        self.left_stick_y = stick.y;
        self.left_stick_button = stick.pressed;
    }
    #[inline]
    fn set_right_stick(&mut self, stick: GamepadStick) {
        self.right_stick_x = stick.x;
        self.right_stick_y = stick.y;
        self.right_stick_button = stick.pressed;
    }
    #[inline]
    fn set_left_trigger(&mut self, trigger: f64) {
        self.left_trigger = trigger;
    }
    #[inline]
    fn set_right_trigger(&mut self, trigger: f64) {
        self.right_trigger = trigger;
    }
    #[inline]
    fn set_a(&mut self, value: bool) {
        self.a = value;
    }
    #[inline]
    fn set_b(&mut self, value: bool) {
        self.b = value;
    }
    #[inline]
    fn set_x(&mut self, value: bool) {
        self.x = value;
    }
    #[inline]
    fn set_y(&mut self, value: bool) {
        self.y = value;
    }
    #[inline]
    fn set_left_bumper(&mut self, value: bool) {
        self.left_bumper = value;
    }
    #[inline]
    fn set_right_bumper(&mut self, value: bool) {
        self.right_bumper = value;
    }
    #[inline]
    fn set_back(&mut self, value: bool) {
        self.back = value;
    }
    #[inline]
    fn set_start(&mut self, value: bool) {
        self.start = value;
    }
}
