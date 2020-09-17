use std::process::Command;
extern crate cpp_build;

fn main() {
    // Only run this build job if the USD source directory has changed
    println!("cargo:rerun-if-changed=thirdparty/USD");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // The out directory of the build
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Generating the wrapper library
    let include = format!("-I{}", include_dir.to_str().unwrap());

    cpp_build::Config::new()
        .include(include_dir.to_str().unwrap())
        .flag("-std=c++14")
        .build("src/lib.rs");
}
