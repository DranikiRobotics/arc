use std::{env, path::{Path, PathBuf}};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    println!("cargo:rustc-link-lib=c++_shared");

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        println!("output_path: {}", &output_path);
        let sysroot_libs_path =
            PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        let lib_path = sysroot_libs_path.join("libc++_shared.so");
        let metadata = std::fs::metadata(&lib_path).unwrap();
        println!("metadata: {:?}", &metadata);
        std::fs::copy(
            lib_path,
            Path::new(&output_path)
                .join("libc++_shared.so"),
        )
        .unwrap();
    }
}