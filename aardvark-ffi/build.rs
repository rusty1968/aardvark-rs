use std::env;
use std::path::PathBuf;
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    //
    // The `rustc-link-lib` instruction tells `Cargo` to link the
    // given library using the compiler's `-l` flag. This is typically
    // used to link a native library using FFI.
    //
    // If you've already add a `#[link(name = "aardvark"]` in the `extern`
    // block, then you don't need to provide this.
    //
    println!("cargo:rustc-link-lib=dylib=aardvark");

    //
    // The `rustc-link-search` instruction tells Cargo to pass the `-L`
    // flag to the compiler to add a directory to the library search path.
    //
    // The optional `KIND` may be one of the values below:
    //
    // - `dependency`: Only search for transitive dependencies in this directory.
    // - `crate`: Only search for this crate's direct dependencies in this directory.
    // - `native`: Only search for native libraries in this directory.
    // - `framework`: Only search for macOS frameworks in this directory.
    // - `all`: Search for all library kinds in this directory. This is the default
    //          if KIND is not specified.
    //
    println!("cargo:rustc-link-search=native=./dynamic-lib");

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
}
