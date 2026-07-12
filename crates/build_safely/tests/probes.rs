use std::{path::PathBuf, process::Command};

use rstest::*;

#[rstest]
/// Runs the tests for each example under `examples/unstable`
///
/// All examples have 2 subdirs `allowed` & `forbidden`, each containing a `.cargo/config.toml` which
/// either specifically allows or forbids the feature. This supports cases where one feature depends
/// on others also being enabled (e.g. unsized_const_params, try_trait_v2).
fn test_unstable(
    #[files("*")]
    #[dirs]
    #[base_dir = "examples/unstable"]
    example: PathBuf,
    #[values(None, Some("allowed"), Some("forbidden"))] config: Option<&str>,
) {
    let mut test = Command::new("cargo");
    test.arg("test").current_dir(&example);
    match config {
        None => {}
        Some(config) => {
            test.env("BUILD_SAFELY_CARGO_CONFIG_DIR", example.join(config));
        }
    };
    let output = test.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    match config {
        Some("forbidden") => assert!(
            stdout.contains("test has_not::"),
            "incorrect tests run: {stdout}"
        ),
        _ => assert!(
            stdout.contains("test has::"),
            "incorrect tests run: {stdout}"
        ),
    };
}
