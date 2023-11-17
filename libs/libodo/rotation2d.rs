use llm::{Float64, Radian64};

pub struct Rotation2D {
    pub real: Float64,
    pub imag: Float64,
}

impl Rotation2D {
    pub const fn new(real: Float64, imag: Float64) -> Self {
        Self { real, imag }
    }
    pub fn log(self) -> Float64 {
        llm::atan2(self.imag, self.real)
    }
}

impl From<libtrig::Angle2D> for Rotation2D {
    fn from(angle: libtrig::Angle2D) -> Self {
        use libtrig::prelude::*;
        Self::new(angle.cos(), angle.sin())
    }
}

impl From<Radian64> for Rotation2D {
    fn from(angle: Radian64) -> Self {
        use libtrig::prelude::*;
        Self::new(angle.cos(), angle.sin())
    }
}

impl core::ops::Neg for Rotation2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(self.real, -self.imag)
    }
}

impl From<Rotation2D> for libtrig::Vec2D {
    fn from(rot: Rotation2D) -> Self {
        Self::new(rot.real, rot.imag)
    }
}

impl core::ops::Mul<Rotation2D> for Rotation2D {
    type Output = Rotation2D;
    fn mul(self, rhs: Rotation2D) -> Self::Output {
        Self::new(
            self.real * rhs.real - self.imag * rhs.imag,
            self.real * rhs.imag + self.imag * rhs.real,
        )
    }
}

impl core::ops::Mul<libtrig::Vec2D> for Rotation2D {
    type Output = libtrig::Vec2D;
    fn mul(self, rhs: libtrig::Vec2D) -> Self::Output {
        libtrig::Vec2D::new(
            self.real * rhs.x() - self.imag * rhs.y(),
            self.real * rhs.y() + self.imag * rhs.x(),
        )
    }
}

impl core::ops::Add<Float64> for Rotation2D {
    type Output = Self;
    fn add(self, rhs: Float64) -> Self::Output {
        self * Self::from(rhs)
    }
}
