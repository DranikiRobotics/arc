use crate::HardwareComponent;
use core::fmt::{Debug, Display};

/// A type that allows sending telemetry data to the driver control station
/// 
/// Although this is a trait, it is not meant to be implemented by the user.
/// Instead, it is implemented by the robot crate.
/// 
/// Even though it is marked as `HardwareComponent`, it is not meant to be loaded.
/// Infact, it will always return `HardwareError::DeviceNotFound` when loaded.
pub trait Telemetry: HardwareComponent {
    /// Sends a debug message to the driver control station
    /// 
    /// It will not be displayed to the driver control station if the robot is not in debug mode.
    fn debug<T: Debug>(&self, message: T);
    /// Sends a message to the driver control station
    /// 
    /// This will always be displayed to the driver control station.
    fn send<T: Display>(&self, message: T);
}
