package dev.matveit.librobotop;

@FunctionalInterface
public interface AccelConstraint {
    MinMax minMaxProfileAccel(Pose2dDual<Arclength> pose, PosePath path, double s);
}
