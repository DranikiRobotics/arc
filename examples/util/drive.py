from arc.math import Vec2D, Vec4D, Radian64, Float64, Angle
from arc.hardware import DcMotor, Gamepad
import arc.math as l2math

from abc import ABC, abstractmethod, abstractstaticmethod
from typing import Generic, TypeVar

MovementVector = TypeVar("MovementVector")
"""A type variable for movement vectors."""

class DriveBase(ABC, Generic[MovementVector]):
    """A base class for drive bases."""
    @abstractmethod
    def move(self, movement: MovementVector) -> None:
        """Drive the robot using a movement vector."""
        raise NotImplementedError("drive() is not implemented for this drive base.")

    @abstractmethod
    def stop(self) -> None:
        """Stop the robot."""
        raise NotImplementedError("stop() is not implemented for this drive base.")

    @abstractstaticmethod
    def calc(*args) -> MovementVector:
        """Calculate the movement vector for a drive base."""
        raise NotImplementedError("calc() is not implemented for this drive base.")

class TankDrive(DriveBase[Vec2D]):
    """A class for tank drive."""
    def __init__(self, left: DcMotor, right: DcMotor):
        """Create a new TankDrive object."""
        self.left = left
        self.right = right

    def move(self, movement: Vec2D):
        self.left.power(movement.x)
        self.right.power(movement.y)

    def stop(self):
        self.move(Vec2D.new(0, 0))
    
    @staticmethod
    def calc(left: Float64, right: Float64) -> Vec2D:
        return Vec2D.new(left, right)

class MechanumDrive(DriveBase[Vec4D]):
    """A class for mechanum drive."""
    def __init__(self, fl: DcMotor, fr: DcMotor, bl: DcMotor, br: DcMotor):
        """Create a new MechanumDrive object."""
        self.fl = fl
        self.fr = fr
        self.bl = bl
        self.br = br

    def move(self, movement: Vec4D):
        self.fl.power(movement.x)
        self.fr.power(movement.y)
        self.bl.power(movement.z)
        self.br.power(movement.w)
    
    def stop(self):
        self.move(Vec4D.new(0, 0, 0, 0))
    
    def moveUsingDefaultSheme(self, gamepad: Gamepad):
        """
        Move the robot using the default scheme.
        The left stick controls movement, and the right stick controls rotation.
        """
        theta = gamepad.left_stick.as_angle()
        power = gamepad.left_stick.as_vec2d().magnitude()
        rotation = gamepad.right_stick.x
        movement = MechanumDrive.calc(theta, power, rotation)
        self.move(movement)
    
    @staticmethod
    def calc(angle: Angle, speed: Float64, turn: Radian64) -> Vec4D:
        sin = l2math.sin(angle.radians() - l2math.FRAC_PI_4)
        cos = l2math.cos(angle.radians() - l2math.FRAC_PI_4)
        mx = max(abs(sin), abs(cos))

        # Calculate the powers for each motor.
        fl = speed * (cos / mx) + turn
        fr = speed * (sin / mx) - turn
        bl = speed * (sin / mx) + turn
        br = speed * (cos / mx) - turn

        # Pump up the power if we can
        if (speed + abs(turn)) > 1:
            fl /= (speed + abs(turn))
            fr /= (speed + abs(turn))
            bl /= (speed + abs(turn))
            br /= (speed + abs(turn))

        # Calculate the final vector.
        return Vec4D.new(fl, fr, bl, br)
