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

def build_llm_bindings(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    architecture = args[0] if len(args) > 0 else get_current_platform()

    print(f"Building L2Math bindings for {architecture}...")

    config = BindingsTargetsConfig(cd)

    print(config.get_all_targets())

    cmd = ["cargo", "build", "--release", "--package", "l2math-bindings"]
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)

    print("Copying L2Math bindings...")
    import shutil, os

    def move_if_needed(name: str) -> bool:
        TO = f"{cd}/libs/l2math/bindings/{name}"
        if os.path.exists(TO):
            print(f"Removing {TO}...")
            os.remove(TO)
        FROM = f"{cd}/target/release/{name}"
        if os.path.exists(FROM) and not os.path.exists(TO):
            shutil.copy(FROM, TO)
            return True
        return False

    if move_if_needed("libl2math_bindings.so") \
    or move_if_needed("l2math_bindings.dll") \
    or move_if_needed("libl2math_bindings.dylib"):
        print("Done!")
        return
    
    return "Could not find the L2Math bindings library!"
