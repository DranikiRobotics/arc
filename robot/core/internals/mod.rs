#![doc = include_str!("./INTERNALS.md")]

use super::*;

pub(crate) mod impls;

/// Metadata used to load a hardware component
#[derive(Debug, Clone)]
pub struct HardwareComponentLoadMetadata {
    pub uuid: HardwareUUID,
}

pub mod builtins {
    use super::*;

    pub type BuiltInHardwareMapImpl = impls::HardwareMapImpl;
    pub type BuiltInTelemetryImpl = impls::TelemetryImpl;
    pub type BuiltInGamepadImpl = impls::GamepadImpl;

    pub type BuiltInDcMotorImpl = impls::DcMotorImpl;
}
