from .util.drive import MechanumDrive
from arc.hardware import *
from arc.math import *
from arc import *

# You are REQUIRED to have a main() function in your program.
# and you MUST NOT call it yourself.
@Auto("Mechanum Drive Example Autonomous")
def main(op: Op):
    op.log("Starting...")

    # Create a MechanumDrive object.
    fl = op.hardwareMap[DcMotor]("motor0")
    fr = op.hardwareMap[DcMotor]("motor1")
    bl = op.hardwareMap[DcMotor]("motor2")
    br = op.hardwareMap[DcMotor]("motor3")
    drive = MechanumDrive(fl, fr, bl, br)

    # Drive forward for 2 seconds.
    movement = drive.calc(Angle.from_degrees(90), 1, 0)
    drive.move(movement)

    sleep(2)

    # Drive right for 2 seconds.
    movement = drive.calc(Angle.from_degrees(0), 1, 0)
    drive.move(movement)

    sleep(2)

    # Stop
    drive.stop()

    op.log("Done!")
    return OK
