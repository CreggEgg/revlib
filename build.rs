use std::path::PathBuf;

fn main() {
    let path = std::path::PathBuf::from("include"); // include path
    // let mut b = autocxx_build::Builder::new("src/main.rs", &[&path])
    //     .extra_clang_args(&["-std=c++20"])
    //     .build()
    //     .unwrap();
    // b.flag_if_supported("-std=c++20") // use "-std:c++17" here if using msvc on windows
    //     .compile("revlib"); // arbitrary library name, pick anything
    let bindings = bindgen::Builder::default()
        .derive_default(true)
        // .derive_ord(true)
        .clang_arg("-Iinclude")
        .header("./include/rev/CANSparkMaxDriver.h")
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: false })
        //.blocklist_type("std::string")
        //.blacklist_type("ctre::phoenix::.*Routines")
        .opaque_type("std::.*")
        //.whitelist_type(SYMBOL_REGEX)
        .allowlist_function("c_.+")
        //.rustified_enum(SYMBOL_REGEX)
        .disable_name_namespacing()
        .ignore_methods()
        .ctypes_prefix("raw")
        .array_pointers_in_arguments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        /*
        .rustfmt_configuration_file(Some(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("rustfmt.toml"),
        ))
        */
        .clang_arg("-xc++")
        .clang_arg("-std=c++14")
        .clang_arg("-idirafter../../Downloads/WPILib_Windows-2024.3.2-artifacts/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12/")
        .clang_arg("-idirafter../../Downloads/WPILib_Windows-2024.3.2-artifacts/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include/c++/12/arm-nilrt-linux-gnueabi/")
        .clang_arg("-idirafter../../Downloads/WPILib_Windows-2024.3.2-artifacts/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/include")
        .generate().unwrap();
        let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=include/*");
    println!("cargo::rustc-link-search=C:/Users/CID/Downloads/WPILib_Windows-2024.3.2-artifacts/roborio/arm-nilrt-linux-gnueabi/sysroot/usr/lib");
    
    let path = PathBuf::from("libs");
    
    println!(
        "cargo:rustc-link-search={}",
        PathBuf::from("libs")
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
    );
    
    println!("cargo::rustc-link-lib=dylib=wpiHal");
    println!("cargo::rustc-link-lib=dylib=REVLibDriver");
    println!("cargo::rustc-link-lib=dylib=wpiutil");
    println!("cargo::rustc-link-lib=dylib=FRC_NetworkCommunication");
    println!("cargo::rustc-link-lib=dylib=RoboRIO_FRC_ChipObject");
    println!("cargo::rustc-link-lib=dylib=visa");
    println!("cargo::rustc-link-lib=dylib=stdc++");
    println!("cargo::rustc-link-lib=dylib=fpgalvshim");
    println!("cargo::rustc-link-lib=dylib=embcanshim");
}
