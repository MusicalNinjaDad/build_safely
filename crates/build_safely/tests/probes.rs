use std::{path::PathBuf, process::Command};

use rstest::*;

struct Setup {
    config_dir: Option<&'static str>,
    channel_override: Option<&'static str>,
    has: bool,
}

mod unstable {
    use super::*;

    const NIGHTLY: Setup = Setup {
        config_dir: None,
        channel_override: Some("+nightly"),
        has: true,
    };

    const NIGHTLY_ALLOWED: Setup = Setup {
        config_dir: Some("allowed"),
        channel_override: Some("+nightly"),
        has: true,
    };

    const NIGHTLY_FORBIDDEN: Setup = Setup {
        config_dir: Some("forbidden"),
        channel_override: Some("+nightly"),
        has: false,
    };

    const STABLE: Setup = Setup {
        config_dir: None,
        channel_override: Some("+stable"),
        has: false,
    };

    const BETA: Setup = Setup {
        config_dir: None,
        channel_override: Some("+beta"),
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
        #[values(NIGHTLY, NIGHTLY_ALLOWED, NIGHTLY_FORBIDDEN, STABLE, BETA)] setup: Setup,
    ) {
        let Setup {
            config_dir,
            channel_override,
            has,
        } = setup;

        let mut test = Command::new("cargo");
        if let Some(channel) = channel_override {
            test.arg(channel);
        };
        test.arg("test")
            .current_dir(&example)
            .env("RUSTC_BOOTSTRAP", "0");
        match config_dir {
            None => {}
            Some(config) => {
                test.env("BUILD_SAFELY_CARGO_CONFIG_DIR", example.join(config));
            }
        };
        let output = test.output().unwrap();
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if has {
            assert!(
                stdout.contains("test has::"),
                "incorrect tests run: {stdout} {stderr}"
            );
        } else {
            assert!(
                stdout.contains("test has_not::"),
                "incorrect tests run: {stdout} {stderr}"
            );
        };
    }
}
