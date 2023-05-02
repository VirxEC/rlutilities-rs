use autocxx_build::Builder;
use glob::glob;
use miette::{IntoDiagnostic, Result};

fn main() -> Result<()> {
    let is_debug = std::env::var("PROFILE").into_diagnostic()?.as_str() == "debug";

    let clang_args = if is_debug { ["-std=c++20"].as_slice() } else { ["-std=c++20", "-flto"].as_slice() };

    let mut builder = Builder::new("src/ctypes.rs", ["RLUtilities-cpp/inc/"]).extra_clang_args(clang_args).build()?;

    // A bug in AutoCXX prevents us from being able to use LTO
    // if !is_debug {
    //     builder.flag_if_supported("-flto").flag_if_supported("/GL");
    // }

    builder
        .use_plt(false)
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
