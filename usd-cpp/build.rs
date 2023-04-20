use std::io::prelude::*;
use std::process::Command;

fn build_cpp_usd(out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // The script directory
    let mut script_dir = std::env::current_dir().unwrap();
    script_dir.push("thirdparty");
    script_dir.push("USD");
    script_dir.push("build_scripts");
    script_dir.push("build_usd.py");

    // The result directory
    let mut cpp_out_dir = std::path::PathBuf::new();
    cpp_out_dir.push(out_dir);
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

fn write_locations(out_dir: &std::path::Path, content: String) {
    // Make sure the source directory exists
    let mut locations_path = out_dir.to_path_buf();
    locations_path.push("locations.rs");
    let mut file = std::fs::File::create(locations_path).unwrap();

    file.write_all(content.as_bytes()).expect("cargo OUT_DIR should be writable");
}

fn write_lib_info(out_dir: &std::path::Path, info: [std::path::PathBuf; 3]) {
    let content = format!(
        "\
pub const INCLUDE : &str = \"{}\"; \n\
pub const LIBS : &str = \"{}\"; \n\
pub const LIB : &str = \"{}\"; \n\
",
        info[0].to_str().unwrap(),
        info[1].to_str().unwrap(),
        info[2].to_str().unwrap(),
    );

    write_locations(out_dir, content);
}

fn write_stub_lib_info(out_dir: &std::path::Path) {
    let content =
        "\
pub const INCLUDE : &str = \"\"; \n\
pub const LIBS : &str = \"\"; \n\
pub const LIB : &str = \"\"; \n\
".to_string();

    write_locations(out_dir, content);
}

fn write_lib_info_from_env(usd_root: &str, out_dir: &std::path::Path) {
    let content = format!(
        "\
pub const INCLUDE : &str = \"{0}/include\"; \n\
pub const LIBS : &str = \"{0}/lib\"; \n\
pub const LIB : &str = \"usd_ms\"; \n\
",
        usd_root
    );

    write_locations(out_dir, content);
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

    if std::env::var("DOCS_RS").is_ok() {
        write_stub_lib_info(&out_dir);
    } else if let Ok(usd_root) = std::env::var("USD_ROOT") {
        write_lib_info_from_env(&usd_root, &out_dir);
    } else {
        // Build the usd cpp library
        write_lib_info(&out_dir, build_cpp_usd(&out_dir));
    }
}
