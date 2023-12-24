"""
ARC Hardware module

This module contains the hardware classes for ARC.
This module includes:
- Gamepads
- Motors
- Sensors
- Servos
"""

from .telemetry import *
from .gamepad import *
from .dcmotor import *
import typing as _typing
import abc as _abc

class HardwareComponent(_abc.ABC):
    """Represents a basic hardware component."""

    @_abc.abstractproperty
    def uuid(self) -> str:
        """The UUID of this hardware component."""
        ...

class HardwareMap(object):
    """
    Represents a map of hardware components.

    This class is used to get hardware components by name.
    """

    HC = _typing.TypeVar("HC", bound = HardwareComponent)    
    def __getitem__(self, type: type[HC]) -> _typing.Callable[[str], HC]:
        """
        Get a hardware component by name.

        This method will return a function that takes a name and returns the hardware component with the specified name.
        If no hardware component with the specified name exists, then this method will raise a KeyError.
        """
        ...
