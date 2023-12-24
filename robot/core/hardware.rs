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
/// # use arc_robot_core as robot;
/// # use robot::HardwareMap;
/// # fn example(hardware_map: impl HardwareMap) -> robot::Result<()> {
/// let motor = hardware_map.load::<robot::hardware::Motor>("motor")?;
/// # Ok(())
/// # }
/// ```
pub trait HardwareMap {
    fn load<C: HardwareComponent>(&self, uuid: impl Into<HardwareUUID>) -> Result<C>;
}
