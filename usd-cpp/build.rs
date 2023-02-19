use std::io::prelude::*;
use std::process::Command;

fn build_tbb(thirdparty: &PathBuf) -> PathBuf {
    // thirdparty
    let mut tbb_src = thirdparty.clone();
    tbb_src.push("oneTBB");

    println!("cargo:rustc-link-lib={}", lib.to_str().unwrap());
    println!(
        "cargo:rustc-link-search=native={}",
        lib_dir.to_str().unwrap()
    );

    [include_dir, lib_dir, lib]
}

fn build_cpp(out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // thirdparty
    let mut thirdparty = std::path::PathBuf::from(
        std::env::current_dir().expect("Unable to obtain the current dir"),
    );
    thirdparty.push("thirdparty");

    build_tbb(&thirdparty);

    // Empty stub paths for the moment
    let include_dir = std::path::PathBuf::new();
    lib_dir = std::path::PathBuf::new();
    lib = std::path::PathBuf::new();

    [include_dir, lib_dir, lib]
}

fn write_lib_info(out_dir: &std::path::PathBuf, info: [std::path::PathBuf; 3]) {
    // Make sure the source directory exists
    let mut locations_path = out_dir.clone();
    locations_path.push("locations.rs");

    write!(
        std::fs::File::create(locations_path).unwrap(),
        "\
pub const INCLUDE : &str = \"{}\"; \n\
pub const LIBS : &str = \"{}\"; \n\
pub const LIB : &str = \"{}\"; \n\
",
        info[0].to_str().unwrap(),
        info[1].to_str().unwrap(),
        info[2].to_str().unwrap(),
    );
}

fn write_stub_lib_info(out_dir: &std::path::PathBuf) {
    // Make sure the source directory exists
    let mut locations_path = out_dir.clone();
    locations_path.push("locations.rs");

    write!(
        std::fs::File::create(locations_path).unwrap(),
        "\
pub const INCLUDE : &str = \"\"; \n\
pub const LIBS : &str = \"\"; \n\
pub const LIB : &str = \"\"; \n\
"
    );
}

fn write_lib_info_from_env(usd_root: &str, out_dir: &std::path::PathBuf) {
    // Make sure the source directory exists
    let mut locations_path = out_dir.clone();
    locations_path.push("locations.rs");

    write!(
        std::fs::File::create(locations_path).unwrap(),
        "\
pub const INCLUDE : &str = \"{0}/include\"; \n\
pub const LIBS : &str = \"{0}/lib\"; \n\
pub const LIB : &str = \"usd_ms\"; \n\
",
        usd_root
    );
}

fn main() {
    // The out directory of the build
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Only run this build job if the thirdparty folder has changed
    println!("cargo:rerun-if-changed=thirdparty");

    if let Ok(_) = std::env::var("DOCS_RS") {
        write_stub_lib_info(&out_dir);
    } else if let Ok(usd_root) = std::env::var("USD_ROOT") {
        write_lib_info_from_env(&usd_root, &out_dir);
    } else {
        // Build the usd cpp library
        write_lib_info(&out_dir, build_cpp(&out_dir));
    }
}
