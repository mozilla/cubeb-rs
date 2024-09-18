use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap();
    let debug = profile.contains("debug");
    if debug {
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmt.lib");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcpmt.lib");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:msvcprt.lib");
        println!("cargo:rustc-link-lib=libcmtd");
        println!("cargo:rustc-link-lib=libcpmtd");
    } else {
        println!("cargo:rustc-link-lib=libcmt");
        println!("cargo:rustc-link-lib=libcpmt");
        // Ignore these if any deps try to pull them in.
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcmtd.lib");
        println!("cargo:rustc-link-arg=/NODEFAULTLIB:libcpmtd.lib");
    }
}

