use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
fn main() {
    let target = env::var("TARGET").unwrap();
    if target.starts_with("thumbv6m") || target.starts_with("thumbv6em") {
        println!("cargo:rustc-cfg=armv6m")
    }

    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    File::create(out.join("device.x"))
        .unwrap()
        .write_all(include_bytes!("device.x"))
        .unwrap();
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=device.x");
    println!("cargo:rerun-if-changed=memory.x");
    println!("cargo:rerun-if-changed=build.rs");
}
