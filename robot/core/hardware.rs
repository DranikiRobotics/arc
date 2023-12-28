//! Hardware components used by the robot

use crate::*;

mod dcmotor;

pub use dcmotor::DcMotor;

/// The hardware map.
/// 
/// This is used to load hardware components.
/// 
/// # Example
/// ```rust
/// # use dranikcore as robot;
/// # use robot::HardwareMap;
/// # fn example(hardware_map: impl HardwareMap) -> robot::Result<()> {
/// let motor = hardware_map.load::<robot::hardware::Motor>("motor")?;
/// # Ok(())
/// # }
/// ```
pub trait HardwareMap {
    /// Loads a hardware component with the given UUID
    fn load<C: HardwareComponent>(&self, uuid: impl Into<HardwareUUID>) -> Result<C>;
}

/// A trait that represents a hardware component
pub trait HardwareComponent: core::fmt::Debug {
    /// Returns the UUID of this component
    #[allow(non_snake_case)]
    fn getUUID(&self) -> HardwareUUID;
    /// Internal method used to load a component
    #[doc(hidden)]
    fn __load_self(_: internals::HardwareComponentLoadMetadata) -> Result<Self> where Self: Sized;
}
