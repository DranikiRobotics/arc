def clean(cd: str, args: list[str]) -> str | int | None:
    import subprocess
    
    DEEP = False
    if '--deep' in args: DEEP = True

    if not DEEP:
        print("Cleaning...")
    else: print("Deep cleaning...")

    cmd = ["cargo", "clean"]
    res = subprocess.run(cmd, cwd=cd)
    if res.returncode != 0: return res.returncode

    if not DEEP: 
        print("Done!")
        return
    
    import shutil, os

    CLEAN_DIRS = ["target", ".build", "__pycache__"]
    CLEAN_DIRS_EXTENSIONS = []
    CLEAN_FILES = ["Cargo.lock"]
    CLEAN_FILES_EXTENSIONS = [".dylib", ".dll", ".lib", ".so"]
    KEEP_FILES = []

    for full_dir_path, _, files in os.walk(cd):
        for file in files:
            if file in CLEAN_FILES:
                if file not in KEEP_FILES:
                    os.remove(os.path.join(full_dir_path, file))
            elif os.path.splitext(file)[1] in CLEAN_FILES_EXTENSIONS:
                if file not in KEEP_FILES:
                    os.remove(os.path.join(full_dir_path, file))
        for dr in CLEAN_DIRS_EXTENSIONS:
            if os.path.basename(full_dir_path).startswith(dr):
                try: shutil.rmtree(full_dir_path)
                except Exception: os.remove(full_dir_path)
        if os.path.basename(full_dir_path) in CLEAN_DIRS:
            shutil.rmtree(full_dir_path)
    
    print("Done!")
