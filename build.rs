use autocxx_build::Builder;
use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    Builder::new("src/ctypes.rs", ["RLUtilities-cpp/inc/"])
        .extra_clang_args(&["-std=c++17"])
        .build()?
        .flag_if_supported("-std=c++17")
        .flag_if_supported("/std:c++17")
        .files(
            glob("RLUtilities-cpp/src/**/*.cc")
                .into_diagnostic()?
                .flatten()
                // don't include the messages folder
                .filter(|p| !p.components().any(|c| c.as_os_str() == "messages")),
        )
        .warnings(false)
        .compile("rlutilities");

    println!("cargo:rerun-if-changed=src/ctypes.rs");

    Ok(())
}
