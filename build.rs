extern crate cmake;
//extern crate bindgen;

//use std::env;
//use std::path::PathBuf;

fn main() {

   //TODO temporary comment
    let dst = cmake::build("libmdbx");

    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=mdbx");

    /*let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("libmdbx/mdbx.h")
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
