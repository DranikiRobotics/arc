from typing import Callable

class Gamepad(object):
    _id: str
    def __repr__(self) -> str:
        return f"<Gamepad {self._id}>"

class Op(object):
    gamepad: Gamepad
    name: str
    def __init__(self, name: str) -> None:
        self.name = name
    def __repr__(self) -> str:
        return f"<Op {self.name}>"

def _stub_op(op: Op) -> None: print("!!!OP IS NOT DEFINED!!!")

_op: Callable[[Op], bool | int | str | None] | None = _stub_op

OK = True
def OP(func: Callable[[Op], bool | int | str | None]) -> None: 
    _op = func

def run_op() -> None:
    op = Op("auto")
    result = _op(op)
    if result == OK: return
    if result is None: return
    
    print(f"Program exited unsucessfully: {result}!")

run_op()
