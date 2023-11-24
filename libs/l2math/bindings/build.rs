use std::env;
use std::path::{Path, PathBuf};

fn main() {
    if match env::var("CARGO_CFG_TARGET_OS") {
        Ok(ref os) if os == "android" => true,
        _ => false,
    } {
        android()
    };
}

fn android() {
    // We expect the NDK to be installed here:
    // toolchains/llvm/prebuilt/{TARGET_TRIPLE}/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so
    println!("cargo:rustc-link-lib=c++_shared");

    let output_path = match env::var("CARGO_NDK_OUTPUT_PATH") {
        Ok(path) => path,
        Err(e) => {
            eprintln!("CARGO_NDK_OUTPUT_PATH not set, using default: {}", e);
            "target".to_string()
        }
    };
    println!("output_path: {}", &output_path);
    let ndk_sysroot_libs_path = match env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH") {
        Some(path) => path,
        None => {
            eprintln!("CARGO_NDK_SYSROOT_LIBS_PATH not set, using default");
            let ndk_path = match env::var("ANDROID_NDK_HOME") {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("ANDROID_NDK_HOME not set, using default: {}", e);
                    "/opt/android-ndk".to_string()
                }
            };
            let target_triple = match env::var("TARGET") {
                Ok(path) => path,
                Err(e) => {
                    eprintln!("TARGET not set, using default: {}", e);
                    "aarch64-linux-android".to_string()
                }
            };
            let ndk_sysroot_path = Path::new(&ndk_path)
                .join("toolchains/llvm/prebuilt")
                .join(&target_triple)
                .join("sysroot");
            let ndk_sysroot_libs_path = ndk_sysroot_path.join("usr/lib");
            ndk_sysroot_libs_path.into()
        }
    };
    println!("ndk_sysroot_libs_path: {:?}", &ndk_sysroot_libs_path);
    let sysroot_libs_path: PathBuf = ndk_sysroot_libs_path.into();
    let lib_path = sysroot_libs_path.join("libc++_shared.so");
    println!("lib_path: {:?}", &lib_path);
    let new_lib_path = Path::new(&output_path).join("libc++_shared.so");
    println!("new_lib_path: {:?}", &new_lib_path);
    if let Err(e) = std::fs::copy(lib_path, new_lib_path) {
        eprintln!("Failed to copy libc++_shared.so: {}", e);
    };
}
