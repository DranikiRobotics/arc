def test(cd: str, args: list[str]) -> str | int | None:

    # tarpaulin_test(cd, args)
    llvm_test(cd, args)

def llvm_test(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    print("Running tests with llvm coverage")

    # Run the tests
    res = subprocess.run(
        ["cargo", "test"],
        cwd=cd,
        env={
            "RUSTFLAGS": "-Cinstrument-coverage",
            "LLVM_PROFILE_FILE": "cargo-test-%p-%m.profraw"
        }
    )

    if res.returncode != 0: exit(res.returncode)
    
    print("Generating coverage report")

    # Generate the coverage report
    def cov(t: str) -> list[str]:
        out = [
            "grcov", ".",
            "--binary-path", "./target/debug/deps/",
            "-s", ".",
            "--branch",
            "--ignore-not-existing",
            "--ignore", "'../*'",
            "--ignore", "'/*'"
        ]
        if t == "html":
            out.extend(["-t", "html", "-o", "target/coverage/html"])
        elif t == "lcov":
            out.extend(["-t", "lcov", "-o", "target/coverage/tests.lcov"])
        else: exit("Invalid coverage type")
        return out
        
    
    # Generate the coverage report
    res = subprocess.run(
        cov("html"),
        cwd=cd
    )

def tarpaulin_test(cd: str, args: list[str]) -> str | int | None:
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
