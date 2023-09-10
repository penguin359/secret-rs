#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
extern crate glib;

use glib::ffi::{GError, GType, GList, GHashTable};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
