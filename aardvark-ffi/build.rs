use std::env;
use std::path::Path;
use std::process::Command;
fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var("OUT_DIR").unwrap();


    // Tell Cargohat if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=src/aardvark.c");
    // Compile the C source file to generate aardvark.o
    cc::Build::new()
        .file("src/aardvark.c") // Specify the C source file
        .out_dir(out_dir.clone())  
        .include("include")
        .compile("aardvark"); // Name the output library "aardvark"

    Command::new("ar").args(&["crus", "libaardvark.a", "aarvark.o"])
    .current_dir(&Path::new(&out_dir))
    .status().unwrap();    
}
