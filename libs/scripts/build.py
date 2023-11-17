def build(cd: str, args: list[str]) -> str | int | None:
    import subprocess
    
    cmd = ["cargo", "build"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)
