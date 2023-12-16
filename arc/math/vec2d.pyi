from .angle import Angle

class Vec2D(object):
    """
    A type representing a 2D vector.
    
    It is usually used in odometry, but can be used for other things as well.
    If you intend to use it to save the position of a robot, you should use
    the `Pos2D` type instead.
    """

    @staticmethod
    def from_angle(angle: Angle, length: float) -> Vec2D:
        """Creates a vector from an angle and a length."""
        ...
    
    @staticmethod
    def from_xy(x: float, y: float) -> Vec2D:
        """Creates a vector from x and y components."""
        ...
    
    def angle(self) -> Angle:
        """Returns the angle of the vector."""
        ...
    
    def length(self) -> float:
        """Returns the length of the vector."""
        ...
    
    def normalize(self) -> Vec2D:
        """Returns a normalized version of the vector."""
        ...
    
    def dot(self, other: Vec2D) -> float:
        """Returns the dot product of two vectors."""
        ...
    
    def __add__(self, other: Vec2D) -> Vec2D:
        """Adds two vectors."""
        ...
    
    def __sub__(self, other: Vec2D) -> Vec2D:
        """Subtracts two vectors."""
        ...
    
    def __mul__(self, other: float) -> Vec2D:
        """Multiplies a vector by a scalar."""
        ...
    
    def __truediv__(self, other: float) -> Vec2D:
        """Divides a vector by a scalar."""
        ...
    
    def __neg__(self) -> Vec2D:
        """Negates a vector."""
        ...
    
    def __eq__(self, other: Vec2D) -> bool:
        """Checks if two vectors are equal."""
        ...
    
    def __ne__(self, other: Vec2D) -> bool:
        """Checks if two vectors are not equal."""
        ...
    
    def __lt__(self, other: Vec2D) -> bool:
        """Checks if the magnitude of one vector is less than another."""
        ...
    
    def __gt__(self, other: Vec2D) -> bool:
        """Checks if the magnitude of one vector is greater than another."""
        ...
    
    def __le__(self, other: Vec2D) -> bool:
        """Checks if the magnitude of one vector is less than or equal to another."""
        ...
    
    def __ge__(self, other: Vec2D) -> bool:
        """Checks if the magnitude of one vector is greater than or equal to another."""
        ...
    
    def __str__(self) -> str:
        """Returns a string representation of the vector."""
        ...
    
    def __repr__(self) -> str:
        """Returns a string representation of the vector."""
        ...
