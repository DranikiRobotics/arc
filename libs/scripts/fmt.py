def fmt(cd: str, args: list[str]) -> str | int | None:
    import subprocess
    
    cmd = ["cargo", "fmt"]
    if '--check' in args: cmd.extend(['--', '--check'])
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)
