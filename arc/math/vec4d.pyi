from . import Float64 as _Float64

class Vec4D(object):
    """
    A type representing a 4D vector.
    
    It is usually used to transmit motion information to the motors.
    """
    
    @staticmethod
    def new(x: _Float64, y: _Float64, z: _Float64, w: _Float64) -> Vec4D:
        """Creates a vector from x, y, z, and w components."""
        ...
    
    @property
    def x(self) -> _Float64:
        """Returns the x component of the vector."""
        ...
    
    @property
    def y(self) -> _Float64:
        """Returns the y component of the vector."""
        ...
    
    @property
    def z(self) -> _Float64:
        """Returns the z component of the vector."""
        ...
    
    @property
    def w(self) -> _Float64:
        """Returns the w component of the vector."""
        ...

    def magnitude(self) -> _Float64:
        """Returns the magnitude of the vector."""
        ...
    
    def normalize(self) -> Vec4D:
        """Returns a new normalized version of this vector."""
        ...
    
    def dot(self, other: Vec4D) -> float:
        """Returns the dot product of two vectors."""
        ...
    
    def __add__(self, other: Vec4D) -> Vec4D:
        """Adds two vectors."""
        ...
    
    def __sub__(self, other: Vec4D) -> Vec4D:
        """Subtracts two vectors."""
        ...
    
    def __mul__(self, other: _Float64) -> Vec4D:
        """Multiplies a vector by a scalar."""
        ...
    
    def __truediv__(self, other: _Float64) -> Vec4D:
        """Divides a vector by a scalar."""
        ...
    
    def __neg__(self) -> Vec4D:
        """Negates a vector."""
        ...
    
    def __eq__(self, other: Vec4D) -> bool:
        """Checks if two vectors are equal."""
        ...
    
    def __ne__(self, other: Vec4D) -> bool:
        """Checks if two vectors are not equal."""
        ...
    
    def __lt__(self, other: Vec4D) -> bool:
        """Checks if the magnitude of one vector is less than another."""
        ...
    
    def __gt__(self, other: Vec4D) -> bool:
        """Checks if the magnitude of one vector is greater than another."""
        ...
    
    def __le__(self, other: Vec4D) -> bool:
        """Checks if the magnitude of one vector is less than or equal to another."""
        ...
    
    def __ge__(self, other: Vec4D) -> bool:
        """Checks if the magnitude of one vector is greater than or equal to another."""
        ...
    
    def __str__(self) -> str:
        """Returns a string representation of the vector."""
        ...
    
    def __repr__(self) -> str:
        """Returns a string representation of the vector. (for debugging)"""
        ...
    
    def __len__(self) -> int:
        """
        Returns the magnitude of the vector.
        
        This is the same as calling `magnitude`.
        """
        ...
