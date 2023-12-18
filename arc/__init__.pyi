"""
### ARC: A complete framework for controlling robots

ARC is a feature complete framework for writing robot code.
It is designed to be easy to use, and easy to understand.
It is also designed to be modular, so that you can use only the parts you need.
Additionally, it is easy to extend, so that you can add your own functionality to it.
"""

from .hardware.gamepad import Gamepad as _Gamepad
import typing as _typing

RunResult = _typing.Union[bool, int, str, None]
"""
This type is used to indicate whether or not an `Op` completed successfully.

If the `Op` completed successfully, then it should return `OK` (or `None`).

If the `Op` did not complete successfully, then it should return an object that
provides information about why it did not complete successfully:
- If it's a `bool`, then it should be False.
- If it's an `int`, then it should be the error code.
- If it's a `str`, then it should be the error message.
"""

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

def Auto(name: str) -> _typing.Callable[[_typing.Callable[[Op], RunResult]], _typing.Callable[[Op], RunResult]]:
    """Decorator for an autonomous operation."""
    ...

def Teleop(name: str) -> _typing.Callable[[_typing.Callable[[Op], RunResult]], _typing.Callable[[Op], RunResult]]:
    """Decorator for a teleop operation."""
    ...

def sleep(seconds: float) -> None:
    """Sleeps for the specified number of seconds."""
    ...

OK: RunResult = True
"""
A built-in `RunResult` that indicates that the `Op` completed successfully.
"""
