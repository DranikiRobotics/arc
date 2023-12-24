"""
Mathematical utilities for arc.

Includes:
- Angle
- Vector2D
- Vector4D
- Position2D
"""

Float64 = float
"""A floating point number with 64 bits of precision."""
Radian64 = float
"""An angle in radians with 64 bits of precision."""
Degree64 = float
"""An angle in degrees with 64 bits of precision."""

EGAMMA: Float64
"""The Euler-Mascheroni constant (γ)"""
FRAC_1_SQRT_3: Float64
"""1/sqrt(3)"""
FRAC_1_SQRT_PI: Float64
"""1/sqrt(π)"""
PHI: Float64
"""The golden ratio (φ)"""
SQRT_3: Float64
"""sqrt(3)"""
E: Float64
"""Euler’s number (e)"""
FRAC_1_PI: Float64
"""1/π"""
FRAC_1_SQRT_2: Float64
"""1/sqrt(2)"""
FRAC_2_PI: Float64
"""2/π"""
FRAC_2_SQRT_PI: Float64
"""2/sqrt(π)"""
FRAC_PI_2: Float64
"""π/2"""
FRAC_PI_3: Float64
"""π/3"""
FRAC_PI_4: Float64
"""π/4"""
FRAC_PI_6: Float64
"""π/6"""
FRAC_PI_8: Float64
"""π/8"""
LN_2: Float64
"""ln(2)"""
LN_10: Float64
"""ln(10)"""
LOG2_10: Float64
"""log2(10)"""
LOG2_E: Float64
"""log2(e)"""
LOG10_2: Float64
"""log10(2)"""
LOG10_E: Float64
"""log10(e)"""
PI: Float64
"""Archimedes’ constant (π)"""
SQRT_2: Float64
"""sqrt(2)"""
TAU: Float64
"""The full circle constant (τ)"""

def acos(x: Float64) -> Radian64:
    """Arccosine"""
    ...
def asin(x: Float64) -> Radian64:
    """Arcsine"""
    ...
def atan(x: Float64) -> Radian64:
    """Arctangent"""
    ...
def atan2(y: Float64, x: Float64) -> Radian64:
    """Arctangent of y/x"""
    ...
def ceil(x: Float64) -> Float64:
    """Ceiling"""
    ...
def cos(x: Radian64) -> Float64:
    """Cosine"""
    ...
def dif(x: Float64, y: Float64) -> Float64:
    """Positive difference"""
    ...
def floor(x: Float64) -> Float64:
    """Floor"""
    ...
def hypot(x: Float64, y: Float64) -> Float64:
    """Calculate the length of the hypotenuse of a right-angle triangle given legs of length x and y."""
    ...
def ln(x: Float64) -> Float64:
    """Natural logarithm"""
    ...
def ln1p(x: Float64) -> Float64:
    """Natural logarithm of 1 plus x"""
    ...
def log(x: Float64) -> Float64:
    """Natural logarithm"""
    ...
def log2(x: Float64) -> Float64:
    """Base 2 logarithm"""
    ...
def log10(x: Float64) -> Float64:
    """Base 10 logarithm"""
    ...
def sin(x: Radian64) -> Float64:
    """Sine"""
    ...
def sqrt(x: Float64) -> Float64:
    """Square root"""
    ...
def tan(x: Radian64) -> Float64:
    """Tangent"""
    ...

from .angle import *
from .vec2d import *
from .vec4d import *
from .pos2d import *

NonStandardFloat = float
"""A floating point number that can be NaN or infinity."""

INFINITY: NonStandardFloat
"""The python representation of infinity."""
NAN: NonStandardFloat
"""The python representation of NaN."""
