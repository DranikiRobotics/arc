def build_llm_bindings(cd: str, args: list[str]) -> str | int | None:
    import subprocess

    print("Building LLM bindings...")
    cmd = ["cargo", "build", "--release", "--package", "llm-bindings"]
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)

    print("Copying LLM bindings...")
    import shutil, os

    def move_if_needed(name: str) -> bool:
        FROM = f"{cd}/target/release/{name}"
        TO = f"{cd}/libs/llm/bindings/{name}"
        if os.path.exists(FROM) and not os.path.exists(TO):
            shutil.copy(FROM, TO)
            return True
        return False

    if move_if_needed("libllm_bindings.so") or move_if_needed("llm_bindings.dll") or move_if_needed("libllm_bindings.dylib"):
        print("Done!")
        return
    
    print("Could not find the LLM bindings library!")
    exit(1)
