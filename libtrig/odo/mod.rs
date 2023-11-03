//! Odometry calculations.

mod movement_calc;
mod config;
mod pos;

pub(crate) use config::OdometryFormulaeConfig;
pub use config::OdometryConfig;

pub use movement_calc::Odometry;
pub use pos::Pos2D;
