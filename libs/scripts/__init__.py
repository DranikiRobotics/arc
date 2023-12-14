from libs.scripts.build import build
from libs.scripts.clean import clean
from libs.scripts.doc import doc
from libs.scripts.fmt import fmt
from libs.scripts.run import run
from libs.scripts.test import test

def run_cmd(cmd: str, cwd: str, args: list[str]) -> str | int | None:
    if cmd == "help" or cmd == "--help" or cmd == "-h": return hlp()
    if cmd == "build": return build(cwd, args)
    if cmd == "clean": return clean(cwd, args)
    if cmd == "doc": return doc(cwd, args)
    if cmd == "fmt": return fmt(cwd, args)
    if cmd == "run": return run(cwd, args)
    if cmd == "test": return test(cwd, args)

    return f"Unknown command: {cmd}"

def hlp() -> str | int | None:
    print("""\033[35mUsage: \033[36m./epearl \033[32m<command> \033[96m[args]\033[0m
\033[35mCommands:
    \033[32mbuild   \033[0mBuild the project
        \033[96m--release\033[0m Build in release mode
    \033[32mrun    \033[0mRun an OpMode
        \033[32m<file>  \033[0mThe file to run
    \033[32mtest    \033[0mRun the tests
        \033[96m--tarpaulin\033[0m Run the tests with tarpaulin
        \033[96m--llvm     \033[0m Run the tests with llvm coverage
        \033[96m--all      \033[0m Run the tests with both tarpaulin and llvm coverage
    \033[32mclean   \033[0mClean the project
        \033[96m--deep   \033[0mRemoves all generated files and folders
    \033[32mdoc     \033[0mGenerate the documentation
        \033[96m--open   \033[0mOpen the documentation in the browser
    \033[32mfmt     \033[0mFix formatting issues in the code
        \033[96m--check  \033[0mCheck if the code is formatted correctly
    \033[32mhelp    \033[0mDisplay this message
\033[35mExamples:
    \033[0mRun the example OpMode: \033[36m./epearl \033[32mrun \033[32m./examples/auto.py
    \033[0mBuild the project: \033[36m./epearl \033[32mbuild \033[96m--release
    \033[0mRun the tests: \033[36m./epearl \033[32mtest \033[96m--all
\033[0m""")
    return