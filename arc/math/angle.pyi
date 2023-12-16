from .vec2d import Vec2D

class Angle(object):
    """
    A type representing an angle.

    It is usually used in odometry, but can be used for other things as well.
    If you intend to use it to save the position of a robot, you should use
    the `Pos2D` type instead.
    """

    @staticmethod
    def from_degrees(degrees: float) -> Angle:
        """Creates an angle from degrees."""
        ...
    
    @staticmethod
    def from_radians(radians: float) -> Angle:
        """Creates an angle from radians."""
        ...
    
    @staticmethod
    def from_vec2d(vec: Vec2D) -> Angle:
        """Creates an angle from a vector."""
        ...
    
    @staticmethod
    def zero() -> Angle:
        """Creates an angle of zero degrees."""
        ...
    
    def degrees(self) -> float:
        """Returns the angle in degrees."""
        ...
    
    def radians(self) -> float:
        """Returns the angle in radians."""
        ...

    def sin(self) -> float:
        """Returns the sine of the angle."""
        ...
    
    def cos(self) -> float:
        """Returns the cosine of the angle."""
        ...
    
    def sqrt(self) -> float:
        """Returns the square root of the angle."""
        ...

    def __add__(self, other: Angle) -> Angle:
        """Adds two angles."""
        ...
    
    def __sub__(self, other: Angle) -> Angle:
        """Subtracts two angles."""
        ...
    
    def __mul__(self, other: float) -> Angle:
        """Multiplies an angle by a scalar."""
        ...
    
    def __truediv__(self, other: float) -> Angle:
        """Divides an angle by a scalar."""
        ...
    
    def __neg__(self) -> Angle:
        """Negates an angle."""
        ...
    
    def __eq__(self, other: Angle) -> bool:
        """Checks if two angles are equal."""
        ...
    
    def __ne__(self, other: Angle) -> bool:
        """Checks if two angles are not equal."""
        ...
    
    def __lt__(self, other: Angle) -> bool:
        """Checks if one angle is less than another."""
        ...
    
    def __le__(self, other: Angle) -> bool:
        """Checks if one angle is less than or equal to another."""
        ...
    
    def __gt__(self, other: Angle) -> bool:
        """Checks if one angle is greater than another."""
        ...
    
    def __ge__(self, other: Angle) -> bool:
        """Checks if one angle is greater than or equal to another."""
        ...
    
    def __str__(self) -> str:
        """Returns a string representation of the angle."""
        ...
    
    def __repr__(self) -> str:
        """Returns a string representation of the angle."""
        ...