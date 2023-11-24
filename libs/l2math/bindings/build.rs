use std::env;
use std::path::{Path, PathBuf};

type Res<T = (), E = Box<dyn std::error::Error>> = Result<T, E>;

fn main() -> Res {
    if env::var("CARGO_CFG_TARGET_OS")? == "android" {
        android()?;
    };
    Ok(())
}

fn android() -> Res {
    println!("cargo:rustc-link-lib=c++_shared");

    let output_path = env::var("CARGO_NDK_OUTPUT_PATH")?;
    println!("output_path: {}", &output_path);
    let ndk_sysroot_libs_path =
        env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").ok_or("CARGO_NDK_SYSROOT_LIBS_PATH not set")?;
    println!("ndk_sysroot_libs_path: {:?}", &ndk_sysroot_libs_path);
    let sysroot_libs_path: PathBuf = ndk_sysroot_libs_path.into();
    let lib_path = sysroot_libs_path.join("libc++_shared.so");
    println!("lib_path: {:?}", &lib_path);
    let new_lib_path = Path::new(&output_path).join("libc++_shared.so");
    println!("new_lib_path: {:?}", &new_lib_path);
    std::fs::copy(lib_path, new_lib_path)?;
    Ok(())
}
