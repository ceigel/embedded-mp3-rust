extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new();
    build
        .include("ffi/minimp3")
        .file("ffi/minimp3.c")
        .compile("minimp3");

    let bb = bindgen::builder()
        .header("ffi/bindgen.h")
        .ctypes_prefix("cty")
        .generate_comments(true)
        .rustfmt_bindings(true)
        .clang_arg("-Iffi/minimp3")
        .clang_arg(format!("--target={}", env::var("HOST").unwrap()))
        .use_core();

    let bindings = bb.generate().expect("Unable to generate bindings");

    //let out_path = PathBuf::from("src");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=bindgen.h");
    println!("cargo:rerun-if-changed=minimp3.c");
}
