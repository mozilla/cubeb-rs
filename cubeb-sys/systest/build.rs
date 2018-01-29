extern crate ctest;

use std::env;
use std::path::PathBuf;

fn main() {
    let root = PathBuf::from(env::var_os("DEP_CUBEB_ROOT").unwrap());

    let mut cfg = ctest::TestGenerator::new();

    // Include the header files where the C APIs are defined
    cfg.header("cubeb.h")
        .header("cubeb_mixer.h")
        .header("cubeb_panner.h")
        .header("cubeb_resampler.h");

    // Include the directory where the header files are defined
    cfg.include(root.join("include"))
        .include(root.join("include/cubeb"))
        .include("../libcubeb/src");

    cfg.type_name(|s, _| s.to_string())
        .field_name(|_, f| match f {
            "device_type" => "type".to_string(),
            _ => f.to_string(),
        });

    // Don't perform `((type_t) -1) < 0)` checks for pointers because
    // they are unsigned and always >= 0.
    cfg.skip_signededness(|s| match s {
        s if s.ends_with("_callback") => true,
        "cubeb_devid" => true,
        _ => false,
    });

    // Generate the tests, passing the path to the `*-sys` library as well as
    // the module to generate.
    cfg.generate("../src/lib.rs", "all.rs");
}
