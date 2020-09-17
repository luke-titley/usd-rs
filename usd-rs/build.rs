fn main() {
    // Explicitly link to the usd cpp library. This should propagate upwards
    // to other libraries
    /*
    println!("cargo:rustc-link-lib={}", usdcpp::LIB);
    println!("cargo:rustc-link-search={}", usdcpp::LIBS);
    */

    // The out directory of the build
    let out_dir = std::env::var("OUT_DIR").unwrap();

    cpp_build::Config::new()
        .include(usdcpp::INCLUDE)
        .flag("-std=c++14")
        .build("src/lib.rs");
}
