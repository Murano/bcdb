extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;


fn main() {
    println!(r"cargo:rustc-link-search=/home/tim/source/libmdbx");

    gcc::Build;
//    TODO makefile copy
    /*let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("mdbx.h")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");*/
}