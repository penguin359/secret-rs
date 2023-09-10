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
        .blocklist_type("GError")
        .blocklist_type("GHashTable")
        .blocklist_type("GList")
        .blocklist_type("GType")
        .allowlist_type("SecretServiceClass")
        .allowlist_type("SecretCollectionClass")
        .allowlist_type("SecretItemClass")
        .allowlist_function("secret_service_get_sync")
        .allowlist_function("secret_service_disconnect")
        .allowlist_function("secret_service_open_sync")
        .allowlist_function("secret_service_get_collections")
        .allowlist_function("secret_service_get_flags")
        .allowlist_function("secret_service_get_session_algorithms")
        .allowlist_function("secret_service_ensure_session_sync")
        .allowlist_function("secret_service_load_collections_sync")
        .allowlist_function("secret_service_search_sync")
        .allowlist_function("secret_service_lock_sync")
        .allowlist_function("secret_service_unlock_sync")
        .allowlist_function("secret_service_store_sync")
        .allowlist_function("secret_service_lookup_sync")
        .allowlist_function("secret_service_clear_sync")
        .allowlist_function("secret_service_prompt_sync")
        .allowlist_function("secret_service_set_alias_sync")
        .allowlist_function("secret_service_get_collection_gtype")
        .allowlist_function("secret_service_get_item_gtype")
        .allowlist_function("secret_service_get_type")
        .allowlist_function("secret_collection_for_alias_sync")
        .allowlist_function("secret_collection_load_items_sync")
        .allowlist_function("secret_collection_create_sync")
        .allowlist_function("secret_collection_search_sync")
        .allowlist_function("secret_collection_delete_sync")
        .allowlist_function("secret_collection_get_created")
        .allowlist_function("secret_collection_get_service")
        .allowlist_function("secret_collection_get_flags")
        .allowlist_function("secret_collection_get_items")
        .allowlist_function("secret_collection_get_label")
        .allowlist_function("secret_collection_set_label_sync")
        .allowlist_function("secret_collection_get_locked")
        .allowlist_function("secret_collection_get_modified")
        .allowlist_function("secret_collection_refresh")
        .allowlist_function("secret_collection_get_type")
        .allowlist_function("secret_item_create_sync")
        .allowlist_function("secret_item_delete_sync")
        .allowlist_function("secret_item_get_schema_name")
        .allowlist_function("secret_item_get_attributes")
        .allowlist_function("secret_item_set_attributes_sync")
        .allowlist_function("secret_item_get_created")
        .allowlist_function("secret_item_get_label")
        .allowlist_function("secret_item_set_label_sync")
        .allowlist_function("secret_item_get_flags")
        .allowlist_function("secret_item_get_locked")
        .allowlist_function("secret_item_get_modified")
        .allowlist_function("secret_item_get_service")
        .allowlist_function("secret_item_get_secret")
        .allowlist_function("secret_item_load_secret_sync")
        .allowlist_function("secret_item_load_secrets_sync")
        .allowlist_function("secret_item_set_secret_sync")
        .allowlist_function("secret_item_refresh")
        .allowlist_function("secret_item_get_type")
        .allowlist_function("secret_value_new")
        .allowlist_function("secret_value_get")
        .allowlist_function("secret_value_get_text")
        .allowlist_function("secret_value_get_content_type")
        .allowlist_function("secret_value_ref")
        .allowlist_function("secret_value_unref")
        .allowlist_function("secret_value_get_type")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
