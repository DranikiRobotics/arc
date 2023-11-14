package dev.matveit.librobotop;

public interface HeadingPath {
    Rotation2dDual<Arclength> get(double s, int n);
    Rotation2dDual<Arclength> begin(int n);
    Rotation2dDual<Arclength> end(int n);
    double length();
}
