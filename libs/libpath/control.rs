use l2math::Float64;
use libtrig::Vec2D;

/// Kinematic motor feedforward
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct MotorFeedforward {
    /// The static gain, or the voltage required to overcome static friction.
    pub k_s: Float64,
    /// The velocity gain, or the voltage required to maintain a given velocity.
    pub k_v: Float64,
    /// The acceleration gain, or the voltage required to produce a given acceleration.
    pub k_a: Float64,
}

impl Eq for MotorFeedforward {}

impl MotorFeedforward {
    /// Creates a new motor feedforward.
    pub const fn new(k_s: Float64, k_v: Float64, k_a: Float64) -> Self {
        Self { k_s, k_v, k_a }
    }
    /// Computes the (normalized) voltage:
    /// ```text,no_run,ignore
    /// |kₛ| · (v / |v|) + kᵥ · v + kₐ · a
    /// ```
    pub fn compute(&self, vel: Float64, accel: Float64) -> Float64 {
        use libtrig::prelude::*;
        self.k_s.signof(vel) + self.k_v * vel + self.k_a * accel
    }
}

/// Proportional position-velocity controller for a holonomic robot.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct HolonomicController {
    axial_pos_gain: Float64,
    lateral_pos_gain: Float64,
    heading_gain: Float64,
    axial_vel_gain: Float64,
    lateral_vel_gain: Float64,
    heading_vel_gain: Float64,
}

impl Eq for HolonomicController {}

impl HolonomicController {
    pub const fn new(
        axial_pos_gain: Float64,
        lateral_pos_gain: Float64,
        heading_gain: Float64,
    ) -> Self {
        Self {
            axial_pos_gain,
            lateral_pos_gain,
            heading_gain,
            axial_vel_gain: 0.0,
            lateral_vel_gain: 0.0,
            heading_vel_gain: 0.0,
        }
    }
    /*
    fun compute(
        targetPose: Pose2dDual<Time>,
        actualPose: Pose2d,
        actualVelActual: PoseVelocity2d,
    ): PoseVelocity2dDual<Time> {
        val targetVelWorld = targetPose.velocity()
        val txActualWorld = Pose2dDual.constant<Time>(actualPose.inverse(), 2)
        val targetVelActual = txActualWorld * targetVelWorld

        val velErrorActual = targetVelActual.value() - actualVelActual

        val error = targetPose.value().minusExp(actualPose)
        return targetVelActual +
            PoseVelocity2d(
                Vector2d(
                    axialPosGain * error.position.x,
                    lateralPosGain * error.position.y,
                ),
                headingGain * error.heading.log(),
            ) +
            PoseVelocity2d(
                Vector2d(
                    axialVelGain * velErrorActual.linearVel.x,
                    lateralVelGain * velErrorActual.linearVel.y,
                ),
                headingVelGain * velErrorActual.angVel,
            )
    }
     */
    // pub fn compute(&self, target: Pose2d, actual: Pose2d, actual_vel: PoseVelocity2d) -> PoseVelocity2d {
    //     let target_vel = target.velocity();
    //     let tx_actual = Pose2d::constant(actual.inverse());
    //     let target_vel_actual = tx_actual * target_vel;
    //     let vel_error_actual = target_vel_actual.value() - actual_vel;
    //     let error = target.value().minus_exp(actual);
    //     target_vel_actual + PoseVelocity2d::new(
    //         Vec2D::new(
    //             self.axial_pos_gain * error.position.x,
    //             self.lateral_pos_gain * error.position.y,
    //         ),
    //         self.heading_gain * error.heading.log(),
    //     ) + PoseVelocity2d::new(
    //         Vec2D::new(
    //             self.axial_vel_gain * vel_error_actual.linear_vel.x,
    //             self.lateral_vel_gain * vel_error_actual.linear_vel.y,
    //         ),
    //         self.heading_vel_gain * vel_error_actual.ang_vel,
    //     )
    // }
}
