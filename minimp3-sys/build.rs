//extern crate bindgen;
extern crate cc;
//use std::env;
//use std::path::PathBuf;

fn main() {
    let mut build = cc::Build::new();
    build
        .include("minimp3")
        .file("minimp3.c")
        .compile("minimp3");

    /*let bb = bindgen::builder()
        .header("bindgen.h")
        .ctypes_prefix("cty")
        .generate_comments(true)
        .rustfmt_bindings(true)
        .clang_arg("-Iminimp3")
        .clang_arg(format!("--target={}", env::var("HOST").unwrap()))
        .use_core();

    // bindgen --output src/bindings.rs bindgen.h --rust-target 1.33 --no-derive-default --ctypes-prefix cty --generate functions,types,vars,methods,constructors,destructors --use-core -- -Iminimp3
    let bindings = bb.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from("src");
    //let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
    println!("cargo:rerun-if-changed=bindgen.h");
    */
    println!("cargo:rerun-if-changed=minimp3.c");
}
