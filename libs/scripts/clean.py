def clean(cd: str, args: list[str]) -> str | int | None:
    import subprocess
    
    DEEP = False
    if '--deep' in args: DEEP = True

    if not DEEP:
        print("Cleaning...")
    else: print("Deep cleaning...")

    cmd = ["cargo", "clean"]
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: exit(res.returncode)

    if not DEEP: 
        print("Done!")
        return
    
    import shutil, os

    CLEAN_DIRS = ["target", ".gradle", ".idea", "out", "build", "__pycache__"]
    CLEAN_EXTENSIONS = [".jar", ".dylib", ".dll", ".lib", ".so"]
    CLEAN_FILES = ["Cargo.lock"]

    KEEP_FILES = ["gradle-wrapper.jar"]

    for full_dir_path, _, files in os.walk(cd):
        for file in files:
            if file in CLEAN_FILES:
                if file not in KEEP_FILES:
                    os.remove(os.path.join(full_dir_path, file))
            elif os.path.splitext(file)[1] in CLEAN_EXTENSIONS:
                if file not in KEEP_FILES:
                    os.remove(os.path.join(full_dir_path, file))
        if os.path.basename(full_dir_path) in CLEAN_DIRS:
            shutil.rmtree(full_dir_path)
    
    print("Done!")
