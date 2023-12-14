def run(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    binary = "arc"
    if len(args) > 0:
        if '--bin=' in args[0]:
            binary = args[0].split('=')[1]
            args = args[1:]

    cmd = ["cargo", "run", "--bin", binary]
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode
