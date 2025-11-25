use glob::glob;
use std::{env, fs, path::PathBuf};

fn main() {
    let target = env::var("TARGET").unwrap_or_default();
    let is_wasm = target.contains("wasm");

    build_lerc(is_wasm);

    if is_wasm {
        // For WASM targets, use pre-generated bindings since bindgen can't parse headers for WASM
        use_pregenerated_bindings();
    } else {
        generate_bindings("vendor/lerc/src/LercLib/include");
    }
}

fn build_lerc(is_wasm: bool) {
    let base = "vendor/lerc/src/LercLib";

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .include(format!("{base}/include"))
        .include(base);

    // Enable C++ exceptions for WASM (required by LERC library)
    if is_wasm {
        build.flag("-fexceptions");
        // Emscripten-specific flags for better WASM output
        build.flag("-sNO_DISABLE_EXCEPTION_CATCHING");
    }

    for entry in glob(&format!("{base}/**/*.cpp")).expect("Failed to read glob pattern") {
        let path = entry.expect("Invalid .cpp path");
        build.file(path);
    }

    build.compile("lerc");

    // Only link stdc++ for non-WASM targets
    if !is_wasm {
        println!("cargo:rustc-link-lib=stdc++");
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={base}");
}

fn use_pregenerated_bindings() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Read pre-generated bindings and adapt for WASM
    let bindings = include_str!("src/bindings_pregenerated.rs");

    // For WASM, replace std::os::raw types with core::ffi equivalents
    let wasm_bindings = bindings
        .replace("::std::os::raw::c_void", "::core::ffi::c_void")
        .replace("::std::os::raw::c_uint", "::core::ffi::c_uint")
        .replace("::std::os::raw::c_int", "::core::ffi::c_int")
        .replace("::std::os::raw::c_uchar", "::core::ffi::c_uchar");

    fs::write(out_path.join("bindings.rs"), wasm_bindings)
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=src/bindings_pregenerated.rs");
}

fn generate_bindings(include_path: &str) {
    let bindings = bindgen::Builder::default()
        .header(format!("{}/Lerc_c_api.h", include_path))
        .clang_arg(format!("-I{}", include_path))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
