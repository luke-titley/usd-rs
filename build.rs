use std::process::Command;

/*
python thirdparty/USD/build_scripts/build_usd.py --build-monolithic --no-tests --no-examples --no-tutorials --no-tools --no-docs --no-python --no-imaging --no-ptex --no-openvdb --no-usdview --no-embree --no-prman --no-openimageio --no-opencolorio --no-alembic --no-hdf5 --no-draco --no-materialx ./
*/

fn build_cpp_usd(out_dir: &str) -> (std::path::PathBuf, std::path::PathBuf) {
    // The script directory
    let mut script_dir = std::path::PathBuf::from(std::env::current_dir().unwrap());
    script_dir.push("thirdparty");
    script_dir.push("USD");
    script_dir.push("build_scripts");
    script_dir.push("build_usd.py");

    // The result directory
    let mut result_dir = std::path::PathBuf::new();
    result_dir.push(&out_dir);
    result_dir.push("result");

    /*
    println!("Downloading and building USD c++ shared library");
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
        .arg(result_dir)
        .current_dir(out_dir)
        .status()
        .unwrap();

    assert!(result.success());
    */

    // The include directory
    let mut include_dir = std::path::PathBuf::new();
    include_dir.push(&result_dir);
    include_dir.push("include");

    // The lib directory
    let mut lib_dir = std::path::PathBuf::new();
    lib_dir.push(&result_dir);
    lib_dir.push("build");
    lib_dir.push("USD");
    lib_dir.push("pxr");
    lib_dir.push("libusd_m.a");

    println!("{:?} {:?}", include_dir, lib_dir);

    (include_dir, lib_dir)
}

fn main() {
    // Only run this build job if the USD source directory has changed
    println!("cargo:rerun-if-changed=thirdparty/USD");
    println!("cargo:rerun-if-changed=include/wrapper.h");

    // The out directory of the build
    let out_dir = std::env::var("OUT_DIR").unwrap();

    // Build the usd cpp library
    let (cpp_include, cpp_lib) = build_cpp_usd(out_dir.as_str());

    // Generating the wrapper library
}
