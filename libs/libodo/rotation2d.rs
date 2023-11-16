use llm::Float64;

pub struct Rotation2D {
    pub real: Float64,
    pub imag: Float64,
}

impl Rotation2D {
    pub const fn new(real: Float64, imag: Float64) -> Self {
        Self { real, imag }
    }
}

impl From<libtrig::Angle2D> for Rotation2D {
    fn from(angle: libtrig::Angle2D) -> Self {
        use libtrig::prelude::*;
        Self::new(angle.cos(), angle.sin())
    }
}
