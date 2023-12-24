"""
Gamepad module.

This module contains the Gamepad class, and related classes.
99% of the time, you will only need the Gamepad class.
"""

from arc.hardware import HardwareComponent as _HardwareComponent

class Gamepad(_HardwareComponent):
    """Represents a gamepad with buttons x, y, a, and b."""

    @property
    def uuid(self) -> str: ...

    @property
    def dpad(self) -> GamepadDpad:
        """The state of the dpad."""
        ...
    
    @property
    def left_stick(self) -> GamepadStick:
        """The state of the left stick."""
        ...
    
    @property
    def right_stick(self) -> GamepadStick:
        """The state of the right stick."""
        ...
    
    @property
    def left_trigger(self) -> float:
        """The state of the left trigger."""
        ...
    
    @property
    def right_trigger(self) -> float:
        """The state of the right trigger."""
        ...

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
    
    @property
    def left_bumper(self) -> bool:
        """Whether or not the left bumper is pressed."""
        ...
    
    @property
    def right_bumper(self) -> bool:
        """Whether or not the right bumper is pressed."""
        ...
    
    @property
    def back(self) -> bool:
        """
        Whether or not the 'back' button is pressed.
        
        This is a non-standard button. Use with caution.
        """
        ...
    
    @property
    def start(self) -> bool:
        """
        Whether or not the 'start' button is pressed.
        
        This is a non-standard button. Use with caution.
        """
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
