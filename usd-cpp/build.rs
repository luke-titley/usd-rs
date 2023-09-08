use std::io::prelude::*;
use std::process::Command;

fn get_outdir() -> std::path::PathBuf {
    std::path::PathBuf::from(
        std::env::var("OUT_DIR").expect("Unable to obtain OUT_DIR env"),
    )
}

fn build_tbb_old(thirdparty: &std::path::PathBuf) {
    let mut out_lib = get_outdir().clone();
    out_lib.push("lib");

    let mut out_include = get_outdir().clone();
    out_include.push("include");

    std::fs::create_dir_all(&out_lib);
    std::fs::create_dir_all(&out_include);

    let mut tbb_lib = out_lib.clone();
    tbb_lib.push("libtbb.a");

    // tbb root
    let mut tbb_root = thirdparty.clone();
    tbb_root.push("oneTBB");
    let mut tbb_src = tbb_root.clone();
    tbb_src.push("src");

    let success = Command::new("make")
        .arg("tbb")
        //.arg("extra_inc=big_iron.inc")
        .arg("-j12")
        .arg(format!("tbb_root={}", tbb_root.display()))
        .arg(format!("tbb_build_prefix=lib"))
        .current_dir(tbb_src)
        .status()
        .expect("Failed to run make for tbb");

    //println!("cargo:rerun-if-changed={}", tbb_root.display());

    assert!(success.success(), "Unable to build tbb");

    // Move the static library
    let mut tbb_built_lib = tbb_root.clone();
    tbb_built_lib.push("build");
    tbb_built_lib.push("lib_release");
    tbb_built_lib.push("libtbb.so");
    std::fs::rename(tbb_built_lib, tbb_lib)
        .expect("Unable to move tbb lib out");

    // Include these boost libraries
    let libraries = ["tbb", "serial"];

    // Loop over the libries we want to copy over
    let mut tbb_include = tbb_root.clone();
    tbb_include.push("include");

    let mut libs_to_copy = Vec::new();
    for name in libraries.iter() {
        let mut path = tbb_include.clone();
        path.push(name);

        println!("cargo:warning=Path {}", path.display());

        libs_to_copy.push(path);
    }

    let options = fs_extra::dir::CopyOptions {
        overwrite: false,
        skip_exist: true,
        buffer_size: 1024,
        copy_inside: true,
        content_only: false,
        depth: 128,
    };
    fs_extra::copy_items(&libs_to_copy, out_include, &options);

    // Copy the cmake module
}

fn build_tbb(thirdparty: &std::path::PathBuf) {
    let mut outdir = get_outdir();

    let mut tbb_root = thirdparty.clone();
    tbb_root.push("oneTBB");

    cmake::Config::new(tbb_root).build();

    let mut lib_folder = outdir.clone();
    lib_folder.push("build");
    lib_folder.push("tbb_cmake_build");
    lib_folder.push("lib_debug"); // copy across release build when appropriate

    let mut lib_path = lib_folder.clone();
    lib_path.push("libtbb.a");
}

fn build_boost_old(thirdparty: &std::path::PathBuf) {
    let mut out_include = get_outdir().clone();
    out_include.push("include");
    out_include.push("boost");

    std::fs::create_dir_all(&out_include);

    // boost root
    let mut boost_root = thirdparty.clone();
    boost_root.push("boost");

    // Include these boost libraries
    let libraries = [
        "any",
        "assign",
        "bind",
        "container",
        "crc",
        "detail",
        "function",
        "functional",
        "iterator",
        "mpl",
        "multi_index",
        "optional",
        "preprocessor",
        "ptr_container",
        "python",
        "random",
        "range",
        "smart_ptr",
        "type_traits",
        "utility",
        "variant",
        "vmd",
    ];

    //"cstdint",
    //"noncopyable",
    //"none",
    //"numeric",
    //"operators",
    //"version",

    // Loop over the libries we want to copy over
    let mut boost_lib = boost_root.clone();
    boost_lib.push("libs");

    let mut libs_to_copy = Vec::new();
    for name in libraries.iter() {
        let mut path = boost_lib.clone();
        path.push(name);
        path.push("include");
        path.push("boost");

        println!("cargo:warning=Path {}", path.display());

        // Loop over all the folders in that directory
        for item in std::fs::read_dir(path).unwrap() {
            let item_path = item.unwrap().path();

            //println!("cargo:warning=    Path {}", item_path.display());

            libs_to_copy.push(item_path);
        }
    }

    let options = fs_extra::dir::CopyOptions {
        overwrite: false,
        skip_exist: true,
        buffer_size: 1024,
        copy_inside: true,
        content_only: false,
        depth: 128,
    };
    fs_extra::copy_items(&libs_to_copy, out_include, &options);
}

fn build_boost(thirdparty: &std::path::PathBuf) {
    let mut outdir = get_outdir().clone();

    // boost root
    let mut boost_root = thirdparty.clone();
    boost_root.push("boost");

    cmake::Config::new(boost_root)
        .build();
}

fn build_usd(thirdparty: &std::path::PathBuf) {
    let mut outdir = get_outdir();

    let mut usd_root = thirdparty.clone();
    usd_root.push("USD");

    /*
        cmake                                       \
        -DTBB_ROOT_DIR=/path/to/tbb                 \
        -DBOOST_ROOT=/path/to/boost                 \
        -DPXR_ENABLE_PTEX_SUPPORT=FALSE
        -DPXR_BUILD_IMAGING=FALSE
        -DPXR_ENABLE_PYTHON_SUPPORT=FALSE
        -DPXR_ENABLE_GL_SUPPORT=FALSE
        -DPXR_ENABLE_METAL_SUPPORT=FALSE
        -DPXR_BUILD_TESTS=FALSE
        -DCMAKE_INSTALL_PREFIX
        /path/to/USD/source

    cmake --build . --target install -- -j <NUM_CORES>
        */

    cmake::Config::new(usd_root)
        .define("TBB_ROOT_DIR", outdir.display().to_string())
        .define("BOOST_ROOT", outdir.display().to_string())
        .define("PXR_ENABLE_PTEX_SUPPORT", "FALSE")
        .define("PXR_BUILD_IMAGING", "FALSE")
        .define("PXR_ENABLE_PYTHON_SUPPORT", "FALSE")
        .define("PXR_BUILD_TESTS", "FALSE")
        .build();
}

fn build_cpp(_out_dir: &std::path::PathBuf) -> [std::path::PathBuf; 3] {
    // thirdparty
    let mut thirdparty = std::path::PathBuf::from(
        std::env::current_dir().expect("Unable to obtain the current dir"),
    );
    thirdparty.push("thirdparty");

    build_tbb(&thirdparty);
    build_boost(&thirdparty);
    build_usd(&thirdparty);

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
    )
    .unwrap();
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
    )
    .unwrap();
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
    )
    .unwrap();
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
