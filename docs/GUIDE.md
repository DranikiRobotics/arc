# Usage Guide

## This will be a step-by-step guide on how to create the [tank_drive.py](../examples/tank_drive.py) example

### Step 1: Import the necessary modules

```python
from arc.hardware import *
from arc import *
```

### Step 2: Create the main function

Note however, that you should not call the `main` function yourself.
It will automatically be called by the library.

```python
def main():
```

For it to be a proper op mode, we add the `@Teleop` decorator to the function.
And add an argument to the function called `op` which will contain all the essential functions for the op mode to run.
That includes items such as `op.gamepad` and `op.telemetry`,
as well as `op.hardwareMap`, which allows us to access the hardware.

```python
@Teleop("My First Op Mode")
def main(op: Op):
```

### Step 3: See if this works by printing something to the console

We can do this by using the `op.log` function.

```python
op.log("Hello World!")
```

### Step 4: Initialize Hardware

This assumes that you have already configured the motors in the config file, read more [here](./GUID_CONFIG_FILE.md).

The config file (we called it `tank_drive.config.jsonc`) should look something like this:

```jsonc
{
    // The name of the config
    "name": "Tank Drive Config",
    // Inputs and outputs
    "io": {
        // hardware component id
        "motor0": {
            // type of hardware component (dcmotor, servo, etc.)
            "type": "dcmotor",
            // port number
            "port": 0
        },
        "motor1": {
            "type": "dcmotor",
            "port": 1
        }
    }
}
```

Now, lets update the annotation to include the config file.

```python
@Teleop("My First Op Mode", config = "tank_drive")
```

And now, we can initialize the hardware by using the `op.hardwareMap` object.

```python
leftMotor  = op.hardwareMap[DcMotor]("motor0")
rightMotor = op.hardwareMap[DcMotor]("motor1")
```

### Step 5: Create the main loop

We will use `op.running` to check if the op mode is still running.

```python
while op.running:
```

### Step 6: Get the gamepad input

Since we are writing a teleop, we can use the gamepad to control the robot.
We can read the gamepad input by using the `op.gamepad` object.

```python
l = op.gamepad.left_stick.y
r = op.gamepad.right_stick.y
```

### Step 7: Set the motor power

With the variables `l` and `r` containing the gamepad input, we can now set the motor power.
And since arc is smart enough to automatically set the motors in reverse when a negative number is passed,
we don't have to worry about writing any extra code.

```python
leftMotor.power  = l
rightMotor.power = r
```

### Full Code

```python
from arc.hardware import *
from arc import *

@Teleop("Tank Drive Example", config = "tank_drive")
def main(op: Op):
    op.log("Starting...")

    # Create a tank drive with the left and right motors.
    leftMotor  = op.hardwareMap[DcMotor]("motor0")
    rightMotor = op.hardwareMap[DcMotor]("motor1")

    # While the opmode is running...
    while op.running:
        # Get the left and right stick values.
        l = op.gamepad.left_stick.y
        r = op.gamepad.right_stick.y
        op.debug("Left: ", l, "Right: ", r)

        # Drive the robot with the left and right sticks.
        leftMotor.power  = l
        rightMotor.power = r

    op.log("Done!")
    return OK

```
