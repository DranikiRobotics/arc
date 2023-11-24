def build(cd: str, args: list[str]) -> str | int | None:
    import subprocess
    
    cmd = ["cargo", "build"]

    if "--android" in args:
        args.remove("--android")
        cmd.extend(["--target", "aarch64-linux-android"])
        cmd.extend(["--features", "__android__"])

    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)
