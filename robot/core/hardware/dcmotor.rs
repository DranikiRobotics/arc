use super::HardwareComponent;
use l2math::Float64;

use crate::*;

/// A simple DC motor
pub trait DcMotor: HardwareComponent {
    /// Sets the power of the motor
    fn set_power(&mut self, power: Float64) -> Result<()>;
}
