#[cfg(feature = "bindgen")]
extern crate bindgen;
#[cfg(all(feature = "vcpkg", target_env = "msvc"))]
extern crate vcpkg;

use std::env;
use std::fs;
use std::path::PathBuf;
// use std::error::Error;
use std::io;

#[cfg(feature = "bindgen")]
use bindgen;

#[cfg(feature = "bindgen")]
fn generate_bindings(include_dir: PathBuf) -> Result<(), io::Error> {
    // TODO: needs more options to match the pregenerated bindings
    let bindings_gen = bindgen::Builder::default().header(include_dir.join("embree3/rtcore.h"));
    let bindings = bindings_gen.generate()?;

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))?;
    Ok(())
}

#[cfg(not(feature = "bindgen"))]
fn generate_bindings(_: PathBuf) -> Result<(), io::Error> {
    Ok(())
}

fn try_load_from_directory(dir: PathBuf, target: &str) -> Result<(), io::Error> {
    if !dir.is_dir() {
        return Err(io::ErrorKind::InvalidInput.into());
    }
    let include_dir = dir.join("include");
    generate_bindings(include_dir)?;

    let bin_dir = dir.join("bin");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // TODO: Windows only
    if target.contains("pc-windows") {
        for &dll in ["embree3.dll", "tbb.dll", "tbb12.dll"].iter() {
            let _ = fs::copy(bin_dir.join(dll), out_path.join(dll));
        }
    }

    println!("cargo:rustc-link-search={}", dir.join("lib").display());

    println!("cargo:rustc-link-search=native={}", out_path.display());

    println!("cargo:rustc-link-lib=embree3");

    // This might cause an issue if another library links to a different version of tbb
    // Ideally there'd be a tbb-sys library that does the linking
    println!("cargo:rustc-link-lib=tbb");

    Ok(())
}

// Don't run the build script when building docs
#[cfg(feature = "docs-rs")]
fn main() {}

#[cfg(not(feature = "docs-rs"))]
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bindings.rs");
    println!("cargo:rerun-if-env-changed=EMBREE_DIR");

    let target = env::var("TARGET").unwrap();

    if let Ok(path) = env::var("EMBREE_DIR") {
        let embree_dir = PathBuf::from(path);

        let r = try_load_from_directory(embree_dir, &target);

        r.expect("Unable to find embree3 in EMBREE_DIR");
    }

    #[cfg(all(feature = "vcpkg", target_env = "msvc"))]
    {
        let vc_pkg = vcpkg::Config::new()
            .emit_includes(true)
            .find_package("embree3");
        if let Ok(lib) = vc_pkg {
            let include_dir = lib.include_paths[0].clone();
            generate_bindings(include_dir).expect("Could not generate bindings");
            return;
        }
    }

    let pkg = pkg_config::Config::new()
        .atleast_version("3.12.0")
        .probe("embree3");
    if let Ok(lib) = pkg {
        let include_dir = lib.include_paths[0].clone();
        generate_bindings(include_dir).expect("Could not generate bindings");

        return;
    }

    // Default install location
    if let Ok(_) = try_load_from_directory(PathBuf::from("C:\\Program Files\\Intel\\Embree3 x64"), &target) {
        return;
    }

    panic!("Couldn't find Embree: set environment variable EMBREE_DIR");

    // if !Path::new("embree/.git").exists() {
    //     Command::new("git").args(&["submodule", "update", "--init"]).status().unwrap();
    // }

    // let _ = Command::new("curl").args(&["-O", "https://github.com/embree/embree/archive/v3.0.0.zip"]).status();
    // let _ = Command::new("tar").args(&["-xf", "v3.0.0.zip", "-", "-C", "embree"]).status();

    // embree_dir = cmake::Config::new("embree")
    // //     .define("EMBREE_ISA_SSE2", "ON")
    // //     .define("EMBREE_ISA_SSE42", "ON")
    // //     .define("EMBREE_ISA_AVX", "ON")
    // //     .define("EMBREE_ISA_AVX2", "ON")
    //     .define("EMBREE_MAX_ISA", "AVX2")
    //     .define("EMBREE_ISPC_SUPPORT", "OFF")
    //     .define("EMBREE_STATIC_LIB", "ON")
    //     .define("EMBREE_TASKING_SYSTEM", "INTERNAL")
    //     .define("EMBREE_TUTORIALS", "OFF")
    //     .build();
}
