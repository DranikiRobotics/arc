"""
### ARC: A complete framework for controlling robots

ARC is a feature complete framework for writing robot code.
It is designed to be easy to use, and easy to understand.
It is also designed to be modular, so that you can use only the parts you need.
Additionally, it is easy to extend, so that you can add your own functionality to it.
"""

from .hardware.gamepad import Gamepad as _Gamepad
from typing import Callable as _Callable

type RunResult = bool | int | str | None

class Op(object):
    """Represents an operation that can be run on the robot."""

    @property
    def gamepad(self) -> _Gamepad:
        """The gamepad that you can use to control the robot."""
        ...

    @property
    def running(self) -> bool:
        """Whether or not the operation is running."""
        ...

def Auto(func: _Callable[[Op], RunResult]) -> _Callable[[Op], RunResult]:
    """Decorator for an autonomous operation."""
    ...

def Teleop(func: _Callable[[Op], RunResult]) -> _Callable[[Op], RunResult]:
    """Decorator for a teleop operation."""
    ...

def sleep(seconds: float) -> None:
    """Sleeps for the specified number of seconds."""
    ...

OK: RunResult = True
