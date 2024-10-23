fn main() {
    let path = std::path::PathBuf::from("include"); // include path
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path])
        .extra_clang_args(&["-std=c++17"])
        .build()
        .unwrap();
    b.flag_if_supported("-std=c++17") // use "-std:c++17" here if using msvc on windows
        .compile("revlib"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");

    println!("cargo::rustc-link-search=./libs");
    println!("cargo::rustc-link-lib=REVLibDriver");
    println!("cargo::rustc-link-lib=dylib=wpiHal");
    println!("cargo::rustc-link-lib=dylib=wpiutil");
}
