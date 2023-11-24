class BindingsTargetsConfig:
    __targets: dict[str, list[str]]
    def __init__(self, cd: str):
        self.__targets = { }

        import tomllib

        cd = f"{cd}/libs/l2math/bindings"

        try:
            with open(f"{cd}/bindings.targets.config.toml", "rb") as f:
                data = tomllib.load(f)
                for target in data:
                    self.add(target, data[target])
        except Exception as e:
            print(f"Failed to load bindings.targets.config.toml: {e}")
            exit(1)

    def add(self, name: str, deps: list[str]):
        self.__targets[name] = deps
    def get(self, name: str) -> list[str]:
        if name not in self.__targets: return []
        return self.__targets[name]
    def get_all_registries(self) -> list[str]:
        return list(self.__targets.keys())
    def get_all_targets(self) -> list[str]:
        out: list[str] = []
        for registry in self.get_all_registries():
            out.extend(self.get(registry))
        return out

# "windows", "linux", "macos", "android", "ios", "unknown"    
def get_current_platform() -> str:
    import platform

    if platform.system() == "Windows":
        return "windows"
    elif platform.system() == "Linux":
        return "linux"
    elif platform.system() == "Darwin":
        return "macos"
    elif platform.system() == "Android":
        return "android"
    elif platform.system() == "iOS":
        return "ios"
    else:
        return "unknown"

def build_l2math_bindings(cd: str, args: list[str]) -> str | int | None:
    architecture = args[0] if len(args) > 0 else get_current_platform()

    print(f"Building L2Math bindings for {architecture}...")

    config = BindingsTargetsConfig(cd)
    targets = config.get(architecture)

    if len(targets) == 0: return f"No targets found for {architecture}!"

    if architecture == "windows":
        return build_l2math_bindings_windows(cd, targets)
    elif architecture == "linux":
        return build_l2math_bindings_linux(cd, targets)
    elif architecture == "macos":
        return build_l2math_bindings_macos(cd, targets)
    elif architecture == "android":
        return build_l2math_bindings_android(cd, targets)
    else:
        return build_l2math_bindings_unknown(cd, targets)

from typing import Callable

def common_build_l2math_bindings(
    cd: str, targets: list[str],
    to_f: Callable[[str], list[str]],
    from_f: Callable[[str], list[str]]
) -> str | None:
    import subprocess, shutil, os

    for target in targets:
        print(f"Building L2Math bindings for {target}...")

        cmd = ["cargo", "build", "--target", target, "-r", "-p", "l2math-bindings"]
        res = subprocess.run(cmd, cwd=cd)
        if res.returncode != 0: exit(res.returncode)

        print("Copying L2Math bindings...")

        TO_DIR = f"{cd}/libs/l2math/bindings/.build"
        if not os.path.exists(TO_DIR):
            os.mkdir(TO_DIR)
        to_f_res = to_f(target)
        from_f_res = from_f(target)

        for i in range(len(to_f_res)):
            TO = f"{TO_DIR}/{to_f_res[i]}"
            if os.path.exists(TO):
                print(f"Removing {TO}...")
                os.remove(TO)
            FROM = f"{cd}/target/{target}/release/{from_f_res[i]}"
            if os.path.exists(FROM) and not os.path.exists(TO):
                shutil.move(FROM, TO)
        print("Done!")
        return
    return "Could not find the L2Math bindings library!"

def build_l2math_bindings_windows(cd: str, targets: list[str]) -> str | None:
    return common_build_l2math_bindings(
        cd, targets,
        lambda t: [f"l2math-{t}.dll", f"l2math-{t}.lib"],
        lambda _: ["l2math_bindings.dll", "l2math_bindings.lib"]
    )

def build_l2math_bindings_linux(cd: str, targets: list[str]) -> str | None:
    return common_build_l2math_bindings(
        cd, targets,
        lambda t: [f"libl2math-{t}.so"],
        lambda _: ["libl2math_bindings.so"]
    )

def build_l2math_bindings_macos(cd: str, targets: list[str]) -> str | None:
    return common_build_l2math_bindings(
        cd, targets,
        lambda t: [f"libl2math-{t}.dylib"],
        lambda _: ["libl2math_bindings.dylib"]
    )

def build_l2math_bindings_android(cd: str, targets: list[str]) -> str | None:
    import shutil, os

    TO_DIR = f"{cd}/libs/l2math/bindings/.build"
    if not os.path.exists(TO_DIR): os.mkdir(TO_DIR)

    for target in targets:
        import subprocess

        print(f"Building L2Math bindings for {target}...")

        cmd = ["cargo", "ndk", "-t", target, "-o", f"{cd}/target/ndk/", "build", "--release", "-p", "l2math-bindings"]
        res = subprocess.run(cmd, cwd = cd)
        if res.returncode != 0: exit(res.returncode)

        print(f"Copying L2Math bindings for {target}...")

        TO = f"{TO_DIR}/libl2math-{target}.so"
        if os.path.exists(TO):
            print(f"Removing {TO}...")
            os.remove(TO)
        FROM = f"{cd}/target/ndk/{target}/libl2math_bindings.so"
        if os.path.exists(FROM) and not os.path.exists(TO):
            shutil.move(FROM, TO)
    
    print("Copying libc++_shared.so...")

    TO = f"{TO_DIR}/libc++_shared.so"
    if os.path.exists(TO):
        print(f"Removing {TO}...")
        os.remove(TO)
    FROM = f"{cd}/target/ndk/libc++_shared.so"
    if os.path.exists(FROM) and not os.path.exists(TO):
        shutil.move(FROM, TO)

    print("Done!")

def build_l2math_bindings_unknown(cd: str, targets: list[str]) -> str | None:
    return "Unknown platform!"
