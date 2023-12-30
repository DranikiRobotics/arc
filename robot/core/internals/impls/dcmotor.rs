use crate::hardware as h;
use crate::*;

struct DcMotorConfig {
    
}

#[derive(Debug, Clone)]
pub struct DcMotorImpl {

}

impl h::HardwareComponent for DcMotorImpl {
    #[allow(non_snake_case)]
    fn getUUID(&self) -> HardwareUUID {
        todo!()
    }

    fn __load_self(_: super::HardwareComponentLoadMetadata) -> Result<Self> where Self: Sized {
        todo!()
    }
    
}

impl h::DcMotor for DcMotorImpl {
    fn set_power(&mut self, power: f64) -> Result<()> {
        todo!()
    }
}
