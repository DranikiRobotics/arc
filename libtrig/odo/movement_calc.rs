use crate::*;

use super::Pos2D;

/// Odometry calculations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Odometry {
    config: odo::OdometryFormulaeConfig,
    current_position: odo::Pos2D,
    encoder_values: Vec3D,
}

impl Odometry {
    /// Creates a new `Odometry` with the given `config`.
    #[must_use]
    pub fn new(config: odo::OdometryConfig) -> Self {
        Self {
            config: config.into(),
            current_position: odo::Pos2D::origin(),
            encoder_values: Vec3D::origin()
        }
    }

    /// Forces the current position to be set to the given `coords`.
    #[inline]
    #[allow(non_snake_case)]
    pub fn setCurrentPosition(&mut self, coords: impl Into<odo::Pos2D>) {
        self.current_position = coords.into();
    }

    /// Forces the current encoder values to be set to the given `values`.
    #[inline]
    #[allow(non_snake_case)]
    pub fn setEncoderValues(&mut self, values: impl Into<Vec3D>) {
        self.encoder_values = values.into();
    }

    /// Resets the current position and encoder values to the origin.
    #[inline]
    #[allow(non_snake_case)]
    pub fn reset(&mut self) {
        self.setCurrentPosition(Pos2D::origin());
        self.setEncoderValues(Vec3D::origin());
    }

    /// Updates the current position
    /// 
    /// Where the `movement` is a `Vec3D` of the raw encoder values.
    /// **NOT THE DELTA ENCODER VALUES.**
    ///
    #[doc = include_str!("./FORMULA.md")]
    pub fn update<V: Into<Vec3D>>(&mut self, encoder_values: V) {
        // Import the `Float` trait. (Needed for the `cos` and `sin` methods.)
        use crate::traits::Float;

        // Converts the encoder values into a `Vec3D`.
        let encoder_values: Vec3D = encoder_values.into();

        // Calculates the delta encoder values.
        let dn1 = encoder_values.x() - self.encoder_values.x();
        let dn2 = encoder_values.y() - self.encoder_values.y();
        let dn3 = encoder_values.z() - self.encoder_values.z();

        // Updates the encoder values.
        self.encoder_values = encoder_values;

        // Calculates the delta x, y, and theta.
        let dtheta = self.config.cm_per_tick * ((dn1 - dn2) / self.config.lateral_wheel_distance);
        let dx = self.config.cm_per_tick * ((dn1 + dn2) / 2.0);
        let dy = self.config.cm_per_tick * (dn3 + ( (dn2 - dn1) / 2.0 ));

        // Updates the current angle by first partition
        let mut new_angle = self.current_position.angle() + (dtheta / 2.0);
        self.current_position.setAngle(new_angle);

        // Updates the current x and y
        let x_incr = dx * new_angle.cos() - dy * new_angle.sin();
        let y_incr = dx * new_angle.sin() + dy * new_angle.cos();
        self.current_position.setX(self.current_position.x() + x_incr);
        self.current_position.setY(self.current_position.y() + y_incr);

        // Updates the current angle by second partition
        new_angle += dtheta / 2.0;
        self.current_position.setAngle(new_angle);
    }

    /// Returns the current position.
    #[inline]
    #[must_use]
    pub fn current_position(&self) -> odo::Pos2D {
        self.current_position
    }
}
