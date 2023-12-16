"""
Gamepad module.

This module contains the Gamepad class, and related classes.
99% of the time, you will only need the Gamepad class.
"""

class Gamepad(object):
    """Represents a gamepad with buttons x, y, a, and b."""

    @property
    def x(self) -> bool:
        """Whether or not the x button is pressed."""
        ...
    @property
    def y(self) -> bool:
        """Whether or not the y button is pressed."""
        ...
    @property
    def a(self) -> bool:
        """Whether or not the a button is pressed."""
        ...
    @property
    def b(self) -> bool:
        """Whether or not the b button is pressed."""
        ...

class GamepadDpad(object):
    """Represents a dpad on a gamepad."""

    @property
    def up(self) -> bool:
        """Whether or not the up button is pressed."""
        ...
    @property
    def down(self) -> bool:
        """Whether or not the down button is pressed."""
        ...
    @property
    def left(self) -> bool:
        """Whether or not the left button is pressed."""
        ...
    @property
    def right(self) -> bool:
        """Whether or not the right button is pressed."""
        ...

class GamepadStick(object):
    """Represents a stick on a gamepad."""

    @property
    def x(self) -> float:
        """The x value of the stick."""
        ...
    @property
    def y(self) -> float:
        """The y value of the stick."""
        ...
    @property
    def pressed(self) -> bool:
        """Whether or not the stick is pressed."""
        ...
    
    from ..math import Angle as _Angle
    from ..math import Vec2D as _Vec2D

    def as_angle(self) -> _Angle:
        """The angle of the stick, in degrees."""
        ...
    
    def as_vec2d(self) -> _Vec2D:
        """The stick as a Vec2D."""
        ...
