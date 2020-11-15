use std::io::prelude::*;
use std::process::Command;

fn copy_headers(out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // The script directory
    let mut source_include_dir =
        std::path::PathBuf::from(std::env::current_dir().unwrap());
    source_include_dir.push("thirdparty");
    source_include_dir.push("docs");
    source_include_dir.push("include");

    // The result directory
    let mut cpp_out_dir = std::path::PathBuf::new();
    cpp_out_dir.push(&out_dir);
    cpp_out_dir.push("install");

    // The include directory
    let mut include_dir = cpp_out_dir.clone();
    include_dir.push("include");

    std::fs::create_dir_all(cpp_out_dir.clone());
    let mut options = fs_extra::dir::CopyOptions::default();
    //options.copy_inside = false;
    options.overwrite = true;
    println!("Copying {:?} to {:?}", source_include_dir, include_dir);
    fs_extra::dir::copy(source_include_dir, cpp_out_dir.clone(),
                        &options).unwrap();

    let lib = std::path::PathBuf::from("");
    let lib_dir = std::path::PathBuf::from("");

    [include_dir, lib_dir, lib]
}

fn build_cpp_usd(out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // The script directory
    let mut script_dir = std::path::PathBuf::from(std::env::current_dir().unwrap());
    script_dir.push("thirdparty");
    script_dir.push("USD");
    script_dir.push("build_scripts");
    script_dir.push("build_usd.py");

    // The result directory
    let mut cpp_out_dir = std::path::PathBuf::new();
    cpp_out_dir.push(&out_dir);
    cpp_out_dir.push("install");

    // The lib directory
    let mut lib_dir = cpp_out_dir.clone();
    lib_dir.push("lib");

    // The include directory
    let mut include_dir = cpp_out_dir.clone();
    include_dir.push("include");

    println!("Downloading dependencies and building USD c++ library");

    // Run the command to build the python c++ library
    let result = Command::new("python")
        .arg(script_dir)
        .arg("--build-monolithic")
        .arg("--no-tests")
        .arg("--no-examples")
        .arg("--no-tutorials")
        .arg("--no-tools")
        .arg("--no-docs")
        .arg("--no-python")
        .arg("--no-imaging")
        .arg("--no-ptex")
        .arg("--no-openvdb")
        .arg("--no-usdview")
        .arg("--no-embree")
        .arg("--no-prman")
        .arg("--no-openimageio")
        .arg("--no-opencolorio")
        .arg("--no-alembic")
        .arg("--no-hdf5")
        .arg("--no-draco")
        .arg("--no-materialx")
        .arg(cpp_out_dir)
        .current_dir(out_dir)
        .status()
        .unwrap();

    assert!(result.success());

    let lib = std::path::PathBuf::from("usd_ms");

    println!("cargo:rustc-link-lib={}", lib.to_str().unwrap());
    println!("cargo:rustc-link-search={}", lib_dir.to_str().unwrap());

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
");
}

fn main() {
    // The out directory of the build
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Only run this build job if the USD source directory has changed
    let mut thirdparty_usd = std::path::PathBuf::from("thirdparty");
    thirdparty_usd.push("USD");
    println!(
        "cargo:rerun-if-changed={}",
        thirdparty_usd.to_str().unwrap()
    );

    if let Ok(_) = std::env::var("DOCS_RS") {
        write_lib_info(&out_dir, copy_headers(&out_dir));
    } else {
        // Build the usd cpp library
        write_lib_info(&out_dir, build_cpp_usd(&out_dir));
    }
}
