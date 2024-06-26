use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    println!("cargo:return-if-changed=src/wrapper.h");
    println!("cargo:rustc-link-lib=pigpiod_if2");

    let binds = bindgen::builder()
        .header("src/wrapper.h")
        .generate()
        .unwrap();

    binds.write_to_file(out_dir.join("bindings.rs")).unwrap();
}
