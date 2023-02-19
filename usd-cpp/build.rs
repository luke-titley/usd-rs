use std::io::prelude::*;
use std::process::Command;

fn get_outdir() -> std::path::PathBuf {
    std::path::PathBuf::from(
        std::env::var("OUT_DIR").expect("Unable to obtain OUT_DIR env"),
    )
}

fn build_tbb(thirdparty: &std::path::PathBuf) {
    let mut tbb_lib = get_outdir().clone();
    tbb_lib.push("libtbb.a");

    // thirdparty
    let mut tbb_root = thirdparty.clone();
    tbb_root.push("oneTBB");
    let mut tbb_src = tbb_root.clone();
    tbb_src.push("src");

    let success = Command::new("make")
        .arg("release")
        .arg("extra_inc=big_iron.inc")
        .arg("-j12")
        .arg(format!("tbb_root={}", tbb_root.display()))
        .arg(format!("tbb_build_prefix=lib"))
        .current_dir(tbb_src)
        .status()
        .expect("Failed to run make for tbb");

    assert!(success.success(), "Unable to build tbb");

    let mut tbb_built_lib = tbb_root.clone();
    tbb_built_lib.push("build");
    tbb_built_lib.push("lib_release");
    tbb_built_lib.push("libtbb.a");
    std::fs::rename(tbb_built_lib, tbb_lib)
        .expect("Unable to move tbb lib out");

    println!("cargo:warning=Install tbb to {}", get_outdir().display())

    //println!("cargo:rustc-link-lib={}", lib.to_str().unwrap());
    //println!(
    //    "cargo:rustc-link-search=native={}",
    //    lib_dir.to_str().unwrap()
    //);
}

fn build_cpp(_out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // thirdparty
    let mut thirdparty = std::path::PathBuf::from(
        std::env::current_dir().expect("Unable to obtain the current dir"),
    );
    thirdparty.push("thirdparty");

    build_tbb(&thirdparty);

    // Empty stub paths for the moment
    let include_dir = std::path::PathBuf::new();
    let lib_dir = std::path::PathBuf::new();
    let lib = std::path::PathBuf::new();

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
    ).unwrap();
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
    ).unwrap();
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
    ).unwrap();
}

fn main() {
    // The out directory of the build
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Only run this build job if the thirdparty folder has changed
    //println!("cargo:rerun-if-changed=thirdparty");

    if let Ok(_) = std::env::var("DOCS_RS") {
        write_stub_lib_info(&out_dir);
    } else if let Ok(usd_root) = std::env::var("USD_ROOT") {
        write_lib_info_from_env(&usd_root, &out_dir);
    } else {
        // Build the usd cpp library
        write_lib_info(&out_dir, build_cpp(&out_dir));
    }
}
