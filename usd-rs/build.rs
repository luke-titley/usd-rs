fn copy_headers(out_dir: &std::path::PathBuf) -> std::path::PathBuf {
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
    options.overwrite = true;
    fs_extra::dir::copy(source_include_dir, cpp_out_dir.clone(), &options)
        .unwrap();

    let lib = std::path::PathBuf::from("");
    let lib_dir = std::path::PathBuf::from("");

    include_dir
}

fn main() {
    if let Ok(_) = std::env::var("DOCS_RS") {
        // The out directory of the build
        let out_dir =
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
        let include_dir = copy_headers(&out_dir);
        println!("cargo:rerun-if-changed={}", include_dir.to_str().unwrap());

        cpp_build::Config::new()
            .include(include_dir)
            .flag("-std=c++14")
            .build("src/lib.rs");
    } else {
        // Explicitly link to the usd cpp library. This should propagate upwards
        // to other libraries
        for lib in usd_cpp::LIBS {
            println!("cargo:rustc-link-lib={}", lib);
        }
        println!("cargo:rustc-link-search={}", usd_cpp::LIB_DIR);

        //panic!("cargo:rustc-link-libs={}", usd_cpp::LIB_DIR);

        // Handle the embedded c++ code
        cpp_build::Config::new()
            .include(usd_cpp::INCLUDE_DIR)
            .flag("-std=c++14")
            .build("src/lib.rs");
    }
}
