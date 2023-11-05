use libtrig::odo::*;
use libtrig::*;

const CONFIG: OdometryConfig = OdometryConfig::new()
    .set_lateral_wheel_distance(10.0)
    .set_longitudinal_wheel_distance(10.0)
    .set_wheel_radius(1.0)
    .set_ticks_per_revolution(100.0);

#[derive(Debug)]
struct Robot(Odometry);

impl Robot {
    fn init() -> Self {
        let mut robot = Self(Odometry::new(CONFIG));
        robot.0.reset();
        robot
    }
    fn update(&mut self, movement: (Float64, Float64, Float64)) {
        self.0.update(Vec3D::from(movement));
    }
}

fn main() {
    let mut robot = Robot::init();

    let mut mv = |x: Float64, y: Float64, theta: Float64| {
        println!("Movement: ({}, {}, {})", x, y, theta);
        robot.update((x, y, theta));
        println!("Pos: {}", robot.0.current_position());
    };

    mv(10.0, 10.0, 0.0);
    mv(100.0, 100.0, 0.0);
    mv(200.0, 000.0, 100.0);
}
