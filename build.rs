// build.rs

fn main() {
    // Compile the C library
    cc::Build::new()
        .file("c_lib/c_lib.c")
        .include("cpp_lib")
        .compile("c_lib");

    // Compile the C++ library
    cc::Build::new()
        .cpp(true)
        .file("cpp_lib/cpp_lib.cpp")
        .compile("cpp_lib");

    // Link the C++ library
    println!("cargo:rustc-link-search=native=./cpp_lib");
    println!("cargo:rustc-link-lib=static=cpp_lib");

    // Link the C library
    println!("cargo:rustc-link-search=native=./c_lib");
    println!("cargo:rustc-link-lib=static=c_lib");

    // Rerun the script if the C or C++ source files change
    println!("cargo:rerun-if-changed=c_lib/c_lib.c");
    println!("cargo:rerun-if-changed=cpp_lib/cpp_lib.cpp");
}
