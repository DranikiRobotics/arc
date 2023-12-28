"""
### ARC: A complete framework for controlling robots

ARC is a feature complete framework for writing robot code.
It is designed to be easy to use, and easy to understand.
It is also designed to be modular, so that you can use only the parts you need.
Additionally, it is easy to extend, so that you can add your own functionality to it.
"""

from .hardware import HardwareMap as _HardwareMap
from .hardware import Telemetry as _Telemetry
from .hardware import Gamepad as _Gamepad
import typing as _typing

RunResult = _typing.Union[bool, int, str, None]
"""
This type is used to indicate whether or not an `Op` completed successfully.

NOTE: This does NOT exist at runtime. It is only used for type checking.

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
    
    @property
    def hardwareMap(self) -> _HardwareMap:
        """The hardware map for the robot."""
        ...
    
    @property
    def telemetry(self) -> _Telemetry:
        """The telemetry system for the robot."""
        ...
    
    def debug(self, *message: object) -> None:
        """
        Sends a debug message using the telemetry system.

        See `op.telemetry.debug` for more information.
        """
        self.telemetry.debug(*message)
    
    def log(self, *message: object) -> None:
        """
        Logs message using the telemetry system.

        See `op.telemetry.log` for more information.
        """
        self.telemetry.log(*message)

__OpFunc = _typing.Callable[[Op], RunResult]
__OpAnnotation = _typing.Callable[[__OpFunc], __OpFunc]

def Auto(name: str, config: str) -> __OpAnnotation:
    """
    Decorator for an autonomous operation.
    
    ### Example:
    ```python
    @Auto("My Auto")
    def main(op: Op):
        op.log("Hello, world!")
    ```
    """
    ...

def Teleop(name: str, config: str) -> __OpAnnotation:
    """
    Decorator for a teleop operation.
    
    ### Example:
    ```python
    @Teleop("My Teleop")
    def main(op: Op):
        op.log("Hello, world!")
    ```
    """
    ...

def sleep(seconds: float) -> None:
    """
    Sleeps for the specified number of seconds.
    
    This is a blocking operation, which means NOTHING will happen until the specified time has passed.
    """
    ...

OK: RunResult = True
"""
A built-in `RunResult` that indicates that the `Op` completed successfully.

See `RunResult` for more information.
"""
