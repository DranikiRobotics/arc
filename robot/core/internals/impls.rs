use std::collections::HashMap;

use super::HardwareComponentLoadMetadata;
use crate::hardware as h;
use crate::prelude::*;

mod dcmotor;
mod gamepad;
pub use dcmotor::DcMotorImpl;
pub use gamepad::GamepadImpl;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HardwareComponentType {
    DcMotor,
}

#[derive(Default, Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct TelemetryImpl;

impl TelemetryImpl {
    pub fn new() -> Self {
        Self {}
    }
}

impl Telemetry for TelemetryImpl {
    fn debug<T: std::fmt::Debug>(&self, message: T) {
        todo!()
    }
    fn send<T: std::fmt::Display>(&self, message: T) {
        todo!()
    }
}

