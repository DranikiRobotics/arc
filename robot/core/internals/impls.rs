use std::collections::HashMap;

use super::HardwareComponentLoadMetadata;
use crate::hardware as h;

mod dcmotor;
mod gamepad;
pub use dcmotor::DcMotorImpl;
pub use gamepad::GamepadImpl;

pub enum HardwareComponentType {
    DcMotor,
}

pub struct HardwareMapImpl {
    components: HashMap<crate::HardwareUUID, HardwareComponentType>,
}

impl HardwareMapImpl {
    pub fn new() -> Self {
        Self { components: HashMap::new() }
    }
}

impl h::HardwareMap for HardwareMapImpl {
    fn load<C: h::HardwareComponent>(&self, uuid: impl Into<crate::HardwareUUID>) -> crate::Result<C> {
        let uuid: crate::HardwareUUID = uuid.into();
        if let Some(component) = self.components.get(&uuid) {
            return C::__load_self(HardwareComponentLoadMetadata {
                uuid: uuid.into(),
            });
        }
        Err(crate::HardwareError::DeviceNotFound)
    }
}
