import subprocess

def run_cmd(cmd: str, cwd: str, args: list[str]) -> str | int | None:
    if cmd == "help" or cmd == "--help" or cmd == "-h": return hlp()
    if cmd == "build": return build(cwd, args)
    if cmd == "test": return test(cwd, args)
    if cmd == "fmt": return fmt(cwd, args)

    return f"Unknown command: {cmd}"

def fmt(cd: str, args: list[str]) -> str | int | None:
    cmd = ["cargo", "fmt"]
    if '--check' in args: cmd.extend(['--', '--check'])
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)

def test(cd: str, args: list[str]) -> str | int | None:
    # Check if tarpaulin is installed
    def is_tarpaulin_installed() -> bool:
        res = subprocess.run(["cargo", "tarpaulin", "--version"], capture_output=True)
        return res.returncode == 0
    
    # Install tarpaulin
    def install_tarpaulin() -> None:
        res = subprocess.run(["cargo", "binstall", "--no-confirm", "cargo-tarpaulin"])
        if res.returncode != 0: exit(res.returncode)
    
    # Check if tarpaulin is installed and install it if it isn't
    if not is_tarpaulin_installed(): install_tarpaulin()

    # Run tarpaulin
    cmd = ["cargo", "tarpaulin"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)

def build(cd: str, args: list[str]) -> str | int | None:
    cmd = ["cargo", "build"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)

def hlp() -> None:
    print("")
    exit(0)