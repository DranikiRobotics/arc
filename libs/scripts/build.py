import subprocess, os

def build(cd: str, args: list[str]) -> str | int | None:
    if "--android" in args:
        args.remove("--android") # Remove the flag
        return build_android(cd, args)
    
    cmd = ["cargo", "build"]
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode

def build_android(cd: str, args: list[str]) -> str | int | None:
    linker = get_android_ndk_linker()
    cmd = [
        "cargo", "build", # Base command
        "--target", "aarch64-linux-android", # Set the target
        "--config", f"target.aarch64-linux-android.linker='{linker}'", # Set the linker
        "--features", "__android__" # Set the features
    ]
    
    # Add the other arguments
    cmd.extend(args)
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode

ANDROID_VERSION = "26"

def get_android_ndk_linker(
    ndk_home: str | None = os.environ.get("ANDROID_NDK_HOME"),
    target: str = "aarch64-linux-android",
    host: str = "linux-x86_64",
) -> str:
    import os

    # Get the NDK path (Thanks to https://github.com/rust-lang/cc-rs/issues/459)
    ndk_home = os.environ.get("ANDROID_NDK_HOME")
    if ndk_home is None: exit("ANDROID_NDK_HOME is not set")

    android_ndk_linker = os.path.join(ndk_home, f"toolchains/llvm/prebuilt/{host}/bin/clang++")
    print("Linker is set to", android_ndk_linker)

    return android_ndk_linker

