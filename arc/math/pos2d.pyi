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
    def from_xyr(x: float, y: float, radians: float) -> Pos2D:
        """Creates a position from x, y, and radians."""
        ...
    
    @staticmethod
    def from_xy(x: float, y: float) -> Pos2D:
        """Creates a position from x and y."""
        ...
    
    @staticmethod
    def from_vec2d(vec: _Vec2D) -> Pos2D:
        """Creates a position from a vector."""
        ...
    
    @staticmethod
    def zero() -> Pos2D:
        """Creates a position positioned at the origin. facing 0 degrees."""
    
    @property
    def x(self) -> float:
        """The x component of the position."""
        ...
    
    @property
    def y(self) -> float:
        """The y component of the position."""
        ...
    
    @property
    def angle(self) -> _Angle:
        """The angle of the position."""
        ...
    
    def move(self, delta: Pos2D) -> None:
        """
        Moves the current position by the specified delta.
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
    