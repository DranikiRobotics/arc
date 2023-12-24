from . import Float64 as _Float64, Radian64 as _Radian64
from .vec2d import Vec2D as _Vec2D
from .angle import Angle as _Angle

class Pos2D(object):
    """
    A type that represents a position in 2D space.
    
    It is usually used to save the position of a robot.
    """

    @staticmethod
    def from_angle_and_pos(angle: _Angle, pos: _Vec2D) -> Pos2D:
        """Creates a position from an angle and a vector."""
        ...
    
    @staticmethod
    def from_xyr(x: _Float64, y: _Float64, radians: _Radian64) -> Pos2D:
        """Creates a position from x, y, and radians."""
        ...
    
    @staticmethod
    def from_xy(x: _Float64, y: _Float64) -> Pos2D:
        """Creates a position from x and y. facing 0 degrees."""
        ...
    
    @staticmethod
    def from_vec2d(vec: _Vec2D) -> Pos2D:
        """Creates a position from a vector. facing 0 degrees."""
        ...
    
    @staticmethod
    def from_angle(angle: _Angle) -> Pos2D:
        """Creates a position from an angle. at the origin."""
        ...
    
    @staticmethod
    def zero() -> Pos2D:
        """Creates a position positioned at the origin. facing 0 degrees."""
    
    @property
    def x(self) -> _Float64:
        """The x component of the position."""
        ...
    
    @property
    def y(self) -> _Float64:
        """The y component of the position."""
        ...
    
    @property
    def angle(self) -> _Angle:
        """The angle of the position."""
        ...
    
    def move(self, delta: Pos2D) -> None:
        """
        Moves the current position by the specified delta.

        This is an impure method, because it mutates the current position.
        """
        ...
    
    def __add__(self, other: Pos2D) -> Pos2D:
        """
        Adds two positions.
        
        The angle of the resulting position is the sum of the angles of the two positions.
        """
        ...
    
    def __sub__(self, other: Pos2D) -> Pos2D:
        """
        Subtracts two positions.
        
        The angle of the resulting position is the difference of the angles of the two positions.

        This is what you should use to find the delta between two positions.
        """
        ...
    
    def __neg__(self) -> Pos2D:
        """
        Negates a position.
        
        The angle also gets negated (rotated 180 degrees).
        """
        ...
    
    def __eq__(self, other: Pos2D) -> bool:
        """Checks if two positions are equal."""
        ...
    
    def __ne__(self, other: Pos2D) -> bool:
        """Checks if two positions are not equal."""
        ...
    