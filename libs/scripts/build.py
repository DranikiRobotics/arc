import subprocess, os

def build(cd: str, args: list[str]) -> str | int | None:
    if "--android" in args:
        print("Android builds are currently halted.")
        print("Hopefully they will be back soon.")
        return 0

    cmd = ["cargo", "build"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode
