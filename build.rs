use autocxx_build::Builder;
use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let mut b = Builder::new("src/ctypes.rs", ["RLUtilities-cpp/inc/"])
        .extra_clang_args(&["-std=c++17"])
        .build()?;

    b.flag_if_supported("-std=c++17")
        .files(
            glob("RLUtilities-cpp/src/**/*.cc")
                .into_diagnostic()?
                .flatten(),
        )
        .include("RLUtilities-cpp/extern/rapidjson/include")
        .warnings(false)
        .compile("rlutilities");

    println!("cargo:rerun-if-changed=src/ctypes.rs");

    Ok(())
}
