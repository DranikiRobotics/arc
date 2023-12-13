from libs.scripts.build import build
from libs.scripts.clean import clean
from libs.scripts.doc import doc
from libs.scripts.test import test
from libs.scripts.fmt import fmt

def run_cmd(cmd: str, cwd: str, args: list[str]) -> str | int | None:
    if cmd == "help" or cmd == "--help" or cmd == "-h": return hlp()
    if cmd == "build": return build(cwd, args)
    if cmd == "clean": return clean(cwd, args)
    if cmd == "doc": return doc(cwd, args)
    if cmd == "test": return test(cwd, args)
    if cmd == "fmt": return fmt(cwd, args)

    return f"Unknown command: {cmd}"

def hlp() -> str | int | None:
    print("""\033[35mUsage: \033[36m./epearl \033[32m<command> \033[96m[args]\033[0m
\033[35mCommands:
    \033[32mbuild   \033[0mBuild the project
    \033[32mclean   \033[0mClean the project
    \033[32mdoc     \033[0mGenerate the documentation
    \033[32mtest    \033[0mRun the tests
    \033[32mfmt     \033[0mFormat the code
    \033[32mhelp    \033[0mDisplay this message""")
    return