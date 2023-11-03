use crate::*;

use core::f64::consts::PI;

/// The configuration for odometry calculations.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct OdometryConfig {
    /// The distance between encoder 1 and encoder 2 in centimeters.
    pub lateral_wheel_distance: float,
    /// The distance between the midpoint of encoder 1 and 2 and encoder 3 in centimeters.
    pub longitudinal_wheel_distance: float,
    /// The radius of the wheels in centimeters.
    pub wheel_radius: float,
    /// The number of ticks per revolution of the encoders.
    pub ticks_per_revolution: float,
}

impl OdometryConfig {
    /// Creates a new `OdometryConfig` with all fields set to `0.0`.
    pub const fn new() -> OdometryConfig {
        OdometryConfig {
            lateral_wheel_distance: 0.0,
            longitudinal_wheel_distance: 0.0,
            wheel_radius: 0.0,
            ticks_per_revolution: 0.0,
        }
    }

    /// Modifies `lateral_wheel_distance` and returns `self`.
    pub const fn set_lateral_wheel_distance(mut self, lateral_wheel_distance: float) -> Self {
        self.lateral_wheel_distance = lateral_wheel_distance;
        self
    }

    /// Modifies `longitudinal_wheel_distance` and returns `self`.
    pub const fn set_longitudinal_wheel_distance(mut self, longitudinal_wheel_distance: float) -> Self {
        self.longitudinal_wheel_distance = longitudinal_wheel_distance;
        self
    }

    /// Modifies `wheel_radius` and returns `self`.
    pub const fn set_wheel_radius(mut self, wheel_radius: float) -> Self {
        self.wheel_radius = wheel_radius;
        self
    }

    /// Modifies `ticks_per_revolution` and returns `self`.
    pub const fn set_ticks_per_revolution(mut self, ticks_per_revolution: float) -> Self {
        self.ticks_per_revolution = ticks_per_revolution;
        self
    }
}

/// The configuration for odometry formulae.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(crate) struct OdometryFormulaeConfig {
    /// The distance between encoder 1 and encoder 2 in centimeters.
    pub lateral_wheel_distance: float,
    /// The distance between the midpoint of encoder 1 and 2 and encoder 3 in centimeters.
    pub longitudinal_wheel_distance: float,
    /// The number of centimeters per tick.
    pub cm_per_tick: float,
}

impl OdometryFormulaeConfig {
    fn cm_per_tick_calc(wheel_radius: float, ticks_per_revolution: float) -> float {
        wheel_radius * 2.0 * PI / ticks_per_revolution
    }
}

impl From<OdometryConfig> for OdometryFormulaeConfig {
    fn from(config: OdometryConfig) -> Self {
        Self {
            lateral_wheel_distance: config.lateral_wheel_distance,
            longitudinal_wheel_distance: config.longitudinal_wheel_distance,
            cm_per_tick: Self::cm_per_tick_calc(config.wheel_radius, config.ticks_per_revolution),
        }
    }
}
