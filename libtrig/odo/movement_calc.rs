use crate::*;

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

    /// Forces the current position to be set to the origin. (0, 0, 0)
    #[inline]
    #[allow(non_snake_case)]
    pub fn setCurrentPositionToOrigin(&mut self) {
        self.current_position = odo::Pos2D::origin();
    }

    /// Updates the current position, with the provided 3D vector
    ///
    #[doc = include_str!("./FORMULA.md")]
    pub fn update(&mut self, movement: Vec3D) {
        let old_encoder_values = self.encoder_values;

        self.encoder_values += movement;

        let dn1 = self.encoder_values.x() - old_encoder_values.x();
        let dn2 = self.encoder_values.y() - old_encoder_values.y();
        let dn3 = self.encoder_values.z() - old_encoder_values.z();

        let dtheta = self.config.cm_per_tick * ((dn1 - dn2) / self.config.lateral_wheel_distance);
        let dx = self.config.cm_per_tick * ((dn1 + dn2) / 2.0);
        let dy = self.config.cm_per_tick * (dn3 + ( (dn2 - dn1) / 2.0 ));

        // pos.h += dtheta / 2;
        // pos.x += dx * Math.cos(pos.h) - dy * Math.sin(pos.h);
        // pos.y += dx * Math.sin(pos.h) + dy * Math.cos(pos.h);
        // pos.h += dtheta / 2;
        // pos.h = normDiff(pos.h);
        // let new_angle = self.current_position.angle() + (dtheta / 2.0);
        // self.current_position.setAngle(new_angle);
        // let new_x = dx * new_angle.cos() - dy * new_angle.sin();
        // self.current_position.setX(new_x);
    }

    /// Returns the current position.
    #[inline]
    #[must_use]
    pub fn current_position(&self) -> odo::Pos2D {
        self.current_position
    }
}
