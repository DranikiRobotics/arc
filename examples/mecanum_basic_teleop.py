from .util.drive import MechanumDrive
from arc.hardware import *
from arc import *

# You are REQUIRED to have a main() function in your program.
# and you MUST NOT call it yourself.
@Teleop("Basic Mechanum Drive Example")
def main(op: Op):
    op.log("Starting...")

    # Create a MechanumDrive object.
    fl = op.hardwareMap[DcMotor]("motor0")
    fr = op.hardwareMap[DcMotor]("motor1")
    bl = op.hardwareMap[DcMotor]("motor2")
    br = op.hardwareMap[DcMotor]("motor3")
    drive = MechanumDrive(fl, fr, bl, br)

    # While the op is running...
    while op.running:
        # Drive the robot using the gamepad.
        drive.moveUsingDefaultSheme(op.gamepad)

    op.log("Done!")
    return OK
