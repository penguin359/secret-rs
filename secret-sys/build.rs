extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    let libs = match pkg_config::find_library("glib-2.0 libsecret-1") {
        Ok(l) => l,
        Err(e) => panic!("{}", e)
    };

    // Handled by pkg_config
    //println!("cargo:rustc-link-lib=libsecret-1");

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(libs.include_paths.iter().map(|x| format!("-I{}", x.as_path().display())))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //.allowlist_function("secret_value_new")
        //.allowlist_function("secret_value_get")
        //.allowlist_function("secret_value_get_text")
        //.allowlist_function("secret_value_get_content_type")
        //.allowlist_function("secret_value_ref")
        .allowlist_function("secret_value_unref")
        //.allowlist_function("secret_value_get_type")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
