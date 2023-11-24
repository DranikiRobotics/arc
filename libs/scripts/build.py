def build(cd: str, args: list[str]) -> str | int | None:
    import subprocess
    
    cmd = ["cargo", "build"]

    if "--android" in args:
        args.remove("--android") # Remove the flag
        cmd.extend(["--target", "aarch64-linux-android"]) # Set the target
        cmd.extend(["--config", "target.aarch64-linux-android.linker='aarch64-linux-android-cc'"]) # Set the linker
        cmd.extend(["--features", "__android__"]) # Set the features
    
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)
