extern crate bindgen;

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();

    println!("cargo:rerun-if-changed=wrapper.h");

    // Copy aardvark.so from dynamic-lib to OUT_DIR
    let src_path = Path::new("dynamic-lib/aardvark.so");
    let dest_path = Path::new(&out_dir).join("aardvark.so");
    fs::copy(src_path, dest_path).expect("Failed to copy aardvark.so to OUT_DIR");

    let bindings = bindgen::Builder::default()
        .layout_tests(true)
        .header("wrapper.h")
        .clang_arg("-I./include")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell Cargohat if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/aardvark.c");
    // Compile the C source file to generate aardvark.o
    cc::Build::new()
        .file("src/aardvark.c") // Specify the C source file
        .out_dir(out_dir.clone())
        .include("include")
        .compile("aardvark");

    Command::new("ar")
        .args(&["crus", "libaardvark.a", "aarvark.o"])
        .current_dir(&Path::new(&out_dir))
        .status()
        .unwrap();
}
