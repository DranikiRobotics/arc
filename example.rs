#![doc = include_str!("./EXAMPLE.md")]
#![doc(hidden)]

use libtrig::odo::*;
use libtrig::*;

const CONFIG: OdometryConfig = OdometryConfig::new()
    .set_lateral_wheel_distance(20.12)
    .set_longitudinal_wheel_distance(11.5)
    .set_wheel_radius(3.0)
    .set_ticks_per_revolution(8192.0);

struct Robot {
    left_encoder: Float64,
    right_encoder: Float64,
    back_encoder: Float64,

    odo: Odometry,
}

impl Robot {
    fn new() -> Self { Self {
        left_encoder: 0.0,
        right_encoder: 0.0,
        back_encoder: 0.0,

        odo: Odometry::new(CONFIG),
    } }
    fn update(&mut self, movement: (Float64, Float64, Float64)) {
        self.left_encoder += movement.0;
        self.right_encoder += movement.1;
        self.back_encoder += movement.2;

        self.odo.update(Vec3D::new(
            self.left_encoder, self.right_encoder, self.back_encoder
        ));
    }
}

fn main() {
    let mut robot = Robot::new();
    robot.odo.setCurrentPositionToOrigin();

    robot.update((10.0, 10.0, 0.0));

    println!("{}", robot.odo.current_position());
}