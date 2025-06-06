// #![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bindgen")]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

#[cfg(not(feature = "bindgen"))]
mod bindings;

pub use self::bindings::*;