def run(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    binary = "arc"
    for i in range(len(args)):
        arg = args[i]
        if '--bin=' in arg:
            binary = arg.split('=')[1]
            args.pop(i)

    cmd = ["cargo", "run", "--bin", binary]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode
