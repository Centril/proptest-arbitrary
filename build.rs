//!
//! This build script detects minimum version and sets
//! flags that are actionable with #[cfg(flag)].
//!

extern crate version_check;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if let Some((true, _)) = version_check::is_min_version("1.24.0") {
        println!("cargo:rustc-cfg=MIN_VER_1_24_0");
    }
}