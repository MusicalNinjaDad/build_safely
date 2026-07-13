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
        runtest(example, setup);
    }
}

mod stable {
    use super::*;

    const STABLE: Setup = Setup {
        config_dir: None,
        channel_override: Some("+stable"),
        has: true,
    };

    const BETA: Setup = Setup {
        config_dir: None,
        channel_override: Some("+beta"),
        has: true,
    };

    const NIGHTLY: Setup = Setup {
        config_dir: None,
        channel_override: Some("+nightly"),
        has: true,
    };

    const PRE_STABILISATION: Setup = Setup {
        config_dir: None,
        channel_override: None,
        has: true,
    };

    const PRE_ALLOWED: Setup = Setup {
        config_dir: Some("allowed"),
        channel_override: None,
        has: true,
    };

    const PRE_FORBIDDEN: Setup = Setup {
        config_dir: Some("forbidden"),
        channel_override: None,
        has: false,
    };

    #[rstest]
    /// Runs the tests for each example under `examples/stable`
    ///
    /// All examples have a rust-toolchain.toml which specifies a nightly channel from before stabilisation.
    ///
    /// All examples have 2 subdirs `allowed` & `forbidden`, each containing a `.cargo/config.toml` which
    /// either specifically allows or forbids the feature. This supports cases where one feature depends
    /// on others also being enabled (e.g. unsized_const_params, try_trait_v2).
    fn examples(
        #[files("*")]
        #[dirs]
        #[base_dir = "examples/stable"]
        example: PathBuf,
        #[values(NIGHTLY, STABLE, BETA, PRE_STABILISATION, PRE_ALLOWED, PRE_FORBIDDEN)]
        setup: Setup,
    ) {
        runtest(example, setup);
    }
}

fn runtest(example: PathBuf, setup: Setup) {
    let Setup {
        config_dir,
        channel_override,
        has,
    } = setup;

    let which_cargo = Command::new("which").arg("cargo").output().unwrap();
    dbg!(which_cargo);

    let mut _cargo_ver = Command::new("sh");
    _cargo_ver.args(["-c","export"]);
    _cargo_ver.current_dir(&example).env("RUSTC_BOOTSTRAP", "0");
    if let Some(config) = config_dir {
        _cargo_ver.env("BUILD_SAFELY_CARGO_CONFIG_DIR", example.join(config));
    };
    dbg!(&_cargo_ver);
    let _cargo_ver_output = _cargo_ver.output().unwrap();
    dbg!(_cargo_ver_output);

    let mut test = Command::new("sh");
    test.args(["-c","cargo"]);
    if let Some(channel) = channel_override {
        test.arg(channel);
    };
    test.current_dir(&example).env("RUSTC_BOOTSTRAP", "0");
    if let Some(config) = config_dir {
        test.env("BUILD_SAFELY_CARGO_CONFIG_DIR", example.join(config));
    };
    test.args(["test", "-vv"]);
    dbg!(&test);

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
