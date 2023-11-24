from libs.l2math.bindings.bindings import build_l2math_bindings

def get_current_target_triplet() -> str | None:
    target_triplet = ""
    import subprocess
    res = subprocess.run(["rustc", "-vV"], capture_output = True)
    if res.returncode != 0: return
    for line in res.stdout.decode().split("\n"):
        if line.startswith("host:"):
            target_triplet = line.split(" ")[1]
            break
    return target_triplet

def test_with_clang(cd: str, args: list[str]) -> str | int | None:
    import subprocess, os

    target_triplet = get_current_target_triplet()
    if target_triplet is None:
        return "Failed to get target triplet"
    
    # Build bindings for current target triplet just in case
    build_l2math_bindings(cd, [target_triplet])

    bin_name = "l2mathtestbin"
    if "windows" in target_triplet:
        bin_name += ".exe"

    BIN_PATH = f"{cd}/libs/l2math/bindings/{bin_name}"
    LIBS_PATH = f"{cd}/libs/l2math/bindings/.build/"

    cmd = [
        "clang++", # Compiler
        "-I", f"{cd}/libs/eztest/", # Include eztest headers
        "-Wall", # Enable all warnings
        "-std=c++14", # Use c++14
        "-L", LIBS_PATH, # Include build directory during linking
        f"-ll2math-{target_triplet}", # Link to l2math-{target_triplet}
        "-o", BIN_PATH, # Output binary
        f"{cd}/libs/l2math/bindings/l2math_test.cpp", # Include test source
        f"{cd}/libs/eztest/eztest.cpp", # Include eztest source
    ]

    #print("cmd:", " ".join(cmd))

    print("Compiling test binary")

    res = subprocess.run(cmd, cwd = cd)
    if res.returncode != 0: return res.returncode

    print("Running test binary")

    res = os.system(BIN_PATH)
    if res != 0: return res

    os.remove(BIN_PATH)

    print("Done!")

def test_with_gcc(cd: str, args: list[str]) -> str | int | None:
    return "GCC is not supported yet"

def test_with_msvc(cd: str, args: list[str]) -> str | int | None:
    return "MSVC is not supported yet"

def bindings_test(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    print("Identifying c++ compiler")
    # Identify the c++ compiler
    res = subprocess.run(
        ["clang++", "--version"],
        capture_output = True
    )

    if res.returncode == 0:
        print("Found clang++")
        return test_with_clang(cd, args)
    
    res = subprocess.run(
        ["g++", "--version"],
        capture_output = True
    )

    if res.returncode == 0:
        print("Found g++")
        return test_with_gcc(cd, args)
    
    res = subprocess.run(
        ["cl", "/?"],
        capture_output = True
    )

    if res.returncode == 0:
        print("Found cl")
        return test_with_msvc(cd, args)
    
    return "No supported c++ compiler found"
