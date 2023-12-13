import subprocess, os

def build(cd: str, args: list[str]) -> str | int | None:
    cmd = ["cargo", "build"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode
