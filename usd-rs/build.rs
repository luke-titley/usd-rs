fn main() {
    // Explicitly link to the usd cpp library. This should propagate upwards
    // to other libraries
    println!("cargo:rustc-link-lib={}", usd_cpp::LIB);
    println!("cargo:rustc-link-search={}", usd_cpp::LIBS);

    cpp_build::Config::new()
        .include(usd_cpp::INCLUDE)
        .flag("-std=c++14")
        .build("src/lib.rs");
}
