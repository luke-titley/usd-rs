use std::process::Command;

/*
python thirdparty/USD/build_scripts/build_usd.py --build-monolithic --no-tests --no-examples --no-tutorials --no-tools --no-docs --no-python --no-imaging --no-ptex --no-openvdb --no-usdview --no-embree --no-prman --no-openimageio --no-opencolorio --no-alembic --no-hdf5 --no-draco --no-materialx ./
*/
fn main() {
    let output = Command::new("python")
        .arg("thirdparty/USD/build_scripts/build_usd.py")
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
        .arg("./")
        .output()
        .expect("Failed to execute command");

    //assert!(false);
}
