use std::{path::PathBuf, process::Command};

use derive_more::Display;
use rstest::*;

use Channel::*;

struct Setup {
    config_dir: Option<&'static str>,
    channel: Channel,
    suffix: Option<&'static str>,
    has: bool,
}

#[derive(Display)]
#[allow(non_camel_case_types)]
enum Channel {
    stable,
    nightly,
}

mod unstable {
    use super::*;

    const NIGHTLY: Setup = Setup {
        config_dir: None,
        channel: nightly,
        suffix: None,
        has: true,
    };

    const NIGHTLY_ALLOWED: Setup = Setup {
        config_dir: Some("allowed"),
        channel: nightly,
        suffix: None,
        has: true,
    };

    const NIGHTLY_FORBIDDEN: Setup = Setup {
        config_dir: Some("forbidden"),
        channel: nightly,
        suffix: None,
        has: false,
    };

    #[rstest]
    /// Runs the tests for each example under `examples/unstable`
    ///
    /// All examples have 2 subdirs `allowed` & `forbidden`, each containing a `.cargo/config.toml` which
    /// either specifically allows or forbids the feature. This supports cases where one feature depends
    /// on others also being enabled (e.g. unsized_const_params, try_trait_v2).
    fn examples(
        #[files("*")]
        #[dirs]
        #[base_dir = "examples/unstable"]
        example: PathBuf,
        #[values(NIGHTLY, NIGHTLY_ALLOWED, NIGHTLY_FORBIDDEN)] setup: Setup,
    ) {
        let Setup {
            config_dir,
            channel,
            suffix,
            has,
        } = setup;

        let toolchain = if let Some(suffix) = suffix {
            format!("+{channel}{suffix}")
        } else {
            format!("+{channel}")
        };

        let mut test = Command::new("cargo");
        test.args([&toolchain, "test"]).current_dir(&example);
        match config_dir {
            None => {}
            Some(config) => {
                test.env("BUILD_SAFELY_CARGO_CONFIG_DIR", example.join(config));
            }
        };
        let output = test.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);

        if has {
            assert!(
                stdout.contains("test has::"),
                "incorrect tests run: {stdout}"
            );
        } else {
            assert!(
                stdout.contains("test has_not::"),
                "incorrect tests run: {stdout}"
            );
        };
    }
}
