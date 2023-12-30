use crate::hardware as h;
use crate::*;

#[derive(Default, Debug, Clone)]
pub struct GamepadImpl {

}

impl GamepadImpl {
    pub(crate) fn new() -> Self {
        Self {

        }
    }
}

impl h::HardwareComponent for GamepadImpl {
    /// Returns the UUID of the gamepad
    /// 
    /// Note: This will always be "gamepad0"
    #[allow(non_snake_case)]
    fn getUUID(&self) -> HardwareUUID {
        HardwareUUID::new(['g', 'a', 'm', 'e', 'p', 'a', 'd', '0'])
    }
    /// Loads the gamepad from the hardware map
    /// 
    /// ### This will always panic
    /// 
    /// This is because the gamepad is not supposed to be loaded from the hardware map.
    fn __load_self(_: super::HardwareComponentLoadMetadata) -> Result<Self> where Self: Sized {
        ::core::panic!("Attempted to load gamepad from hardware map");
    }
}

impl crate::prelude::Gamepad for GamepadImpl {
    fn dpad(&self) -> crate::Result<gamepad::GamepadDpad> {
        todo!()
    }

    fn left_stick(&self) -> crate::Result<gamepad::GamepadStick> {
        todo!()
    }

    fn right_stick(&self) -> crate::Result<gamepad::GamepadStick> {
        todo!()
    }

    fn left_trigger(&self) -> crate::Result<l2math::Float64> {
        todo!()
    }

    fn right_trigger(&self) -> crate::Result<l2math::Float64> {
        todo!()
    }

    fn a(&self) -> crate::Result<bool> {
        todo!()
    }

    fn b(&self) -> crate::Result<bool> {
        todo!()
    }

    fn x(&self) -> crate::Result<bool> {
        todo!()
    }

    fn y(&self) -> crate::Result<bool> {
        todo!()
    }

    fn left_bumper(&self) -> crate::Result<bool> {
        todo!()
    }

    fn right_bumper(&self) -> crate::Result<bool> {
        todo!()
    }
}

impl crate::gamepad::MutableGamepad for GamepadImpl {
    fn set_dpad(&mut self, _: gamepad::GamepadDpad) -> crate::Result<()> {
        todo!()
    }

    fn set_left_stick(&mut self, _: gamepad::GamepadStick) -> crate::Result<()> {
        todo!()
    }

    fn set_right_stick(&mut self, _: gamepad::GamepadStick) -> crate::Result<()> {
        todo!()
    }

    fn set_left_trigger(&mut self, _: l2math::Float64) -> crate::Result<()> {
        todo!()
    }

    fn set_right_trigger(&mut self, _: l2math::Float64) -> crate::Result<()> {
        todo!()
    }

    fn set_a(&mut self, _: bool) -> crate::Result<()> {
        todo!()
    }

    fn set_b(&mut self, _: bool) -> crate::Result<()> {
        todo!()
    }

    fn set_x(&mut self, _: bool) -> crate::Result<()> {
        todo!()
    }

    fn set_y(&mut self, _: bool) -> crate::Result<()> {
        todo!()
    }

    fn set_left_bumper(&mut self, _: bool) -> crate::Result<()> {
        todo!()
    }

    fn set_right_bumper(&mut self, _: bool) -> crate::Result<()> {
        todo!()
    }
}
