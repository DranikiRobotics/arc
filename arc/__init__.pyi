from typing import Callable

type RunResult = bool
OK: RunResult = True
FAIL: RunResult = False
def MAIN(func: Callable[[], RunResult]) -> None: ...
