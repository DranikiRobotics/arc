def test(cd: str, args: list[str]) -> str | int | None:
    import subprocess

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
