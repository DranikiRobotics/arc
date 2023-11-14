package dev.matveit.librobotop;

public class MinMax {
    public final double min;
    public final double max;
    public MinMax(double min, double max) {
        this.min = min;
        this.max = max;
    }
    public MinMax(MinMax other) {
        this(other.min, other.max);
    }
}
