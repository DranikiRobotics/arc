def test(cd: str, args: list[str]) -> str | int | None:
    if len(args) == 0: return "No test type specified"
    if 'tarpaulin' == args[0]:
        args.pop(0)
        return tarpaulin_test(cd, args)
    elif 'llvm' == args[0]:
        args.pop(0)
        return llvm_test(cd, args)
    elif 'all' == args[0]:
        args.pop(0)
        tarpaulin_test_res = tarpaulin_test(cd, args)
        if tarpaulin_test_res != None: return tarpaulin_test_res
        llvm_test_res = llvm_test(cd, args)
        if llvm_test_res != None: return llvm_test_res
    else: return "Invalid test type"

def llvm_test(cd: str, args: list[str]) -> str | int | None:
    print("LLVM tests are currently broken")
    return

    import subprocess

    print("Running tests with llvm coverage")

    # Run the tests
    res = subprocess.run(
        ["cargo", "test"],
        cwd = cd,
        env = {
            "RUSTFLAGS": "-Cinstrument-coverage",
            "LLVM_PROFILE_FILE": "./target/cargo-test-%p-%m.profraw"
        }
    )

    if res.returncode != 0: return res.returncode
    
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
        else: return "Invalid coverage type"
        return out
        
    
    # Generate the coverage report
    res = subprocess.run(
        cov("html"),
        cwd = cd
    )

def tarpaulin_test(cd: str, args: list[str]) -> int | str | None:
    import subprocess

    # Check if tarpaulin is installed
    def is_tarpaulin_installed() -> bool:
        res = subprocess.run(["cargo", "tarpaulin", "--version"], capture_output=True)
        return res.returncode == 0
    
    # Install tarpaulin
    def install_tarpaulin() -> int | None:
        res = subprocess.run(["cargo", "binstall", "--no-confirm", "cargo-tarpaulin"])
        if res.returncode != 0: return res.returncode
    
    # Check if tarpaulin is installed and install it if it isn't
    if not is_tarpaulin_installed(): 
        res = install_tarpaulin()
        if res != None: return res

    # Run tarpaulin
    cmd = ["cargo", "tarpaulin"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode
