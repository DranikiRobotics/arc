from libs.scripts.bindings import bindings
from libs.scripts.build import build
from libs.scripts.clean import clean
from libs.scripts.test import test
from libs.scripts.fmt import fmt

def run_cmd(cmd: str, cwd: str, args: list[str]) -> str | int | None:
    if cmd == "help" or cmd == "--help" or cmd == "-h": return hlp()
    if cmd == "bindings": return bindings(cwd, args)
    if cmd == "build": return build(cwd, args)
    if cmd == "clean": return clean(cwd, args)
    if cmd == "test": return test(cwd, args)
    if cmd == "fmt": return fmt(cwd, args)

    return f"Unknown command: {cmd}"

def hlp() -> None:
    print("")
    exit(0)