def doc(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    cmd = ["cargo", "doc", "--no-deps"]

    PACKAGES = [
        "libodo",
        "libpath",
        "libtrig",
        "llm",
        "macros"
    ]

    for p in PACKAGES:
        cmd.append("--package")
        cmd.append(p)

    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode
