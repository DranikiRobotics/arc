from typing import Callable

type RunResult = bool | int | str | None
class Gamepad(object):
    pass
    
class Op(object):
    gamepad: Gamepad
    name: str
    def __init__(self, name: str) -> None: ...
    def __repr__(self) -> str: ...
OK: RunResult = True
def OP(func: Callable[[Op], RunResult]) -> None: """
Run the given function as the main function of the program.
"""
