In a 3-wheel odometry setup,
where two deadwheels are parallel and the third wheel intersects the two parallel wheels,
we can use this configuration to estimate the robot's position and orientation as it moves on the field.

<img
    src="https://cdn.statically.io/gh/NoahBres/LearnRoadRunner/1c0fe8d5/docs/assets/dead-wheels/andrew-bot-forward-offset-quarter.jpg"
    alt="Basic Robot Encoder Setup"
    width="100%"
    height="auto"
    decoding="async">
</img>

Input:

- `FLCount` - Encoder count for the front-left dead wheel. (yes we are using encoder)
- `FRCount` - Encoder count for the front-right dead wheel.
- `ICount` - Encoder count for the intersecting dead wheel.
- `DCLateral` - Distance calibration factor for lateral movement (strafe).
                This factor relates encoder counts to lateral distance traveled for each wheel.
- `DCForward` - Distance calibration factor for forward/backward movement.
                This factor relates encoder counts to forward/backward distance traveled for each wheel.
- `DCAngle` - Angle calibration factor. This factor relates encoder counts to the angle of rotation for the robot.
- `x` - Robot's x-coordinate on the field (lateral position).
- `y` - Robot's y-coordinate on the field (forward position).
- `θ` - Robot's orientation with respect to a reference direction (usually the starting orientation)

Update Loop:
The robot continuously reads encoder counts from the odometry wheels at regular intervals,
typically every few milliseconds. For each loop iteration,
the robot updates its position and orientation on the field.

Lateral Movement Update:
The change in lateral position (Δx)
can be calculated using the average encoder counts from the
front-left, front-right, and intersecting dead wheels:

```rust,ignore,no_run
    Δx = (FLCount + FRCount + ICount) * DCLateral / 3
```

Forward Movement Update: The change in forward position (Δy)
can be calculated using the average encoder counts from the two parallel dead wheels:

```rust,ignore,no_run
    Δy = (FLCount + FRCount) * DCForward / 2
```

Updating Orientation: The change in orientation (Δθ)
can be calculated using the difference in encoder counts between the front-left and front-right dead wheels.
The distance between these wheels is known as the "track width" (W):

```rust,ignore,no_run
    Δθ = (FLCount - FRCount) * DCAngle / W
```

Combining Updates:
Using the `Δx`, `Δy`, and `Δtheta` calculated in the previous steps,
the robot can update its position and orientation as follows:

```rust,ignore,no_run
    xₙ = xₒ + Δy * cos(θ) - Δx * sin(θ)
    yₙ = yₒ + Δy * sin(θ) + Δx * cos(θ)
    θₙ = θₒ + Δθ
```
