use std::{fs, path::PathBuf, process::Command};

use rstest::*;
use serde::Deserialize;
use toml::Table;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Setup {
    config_dir: Option<&'static str>,
    channel_override: Option<&'static str>,
    has: bool,
}

mod unstable {
    use super::*;

    const NIGHTLY: Setup = Setup {
        config_dir: None,
        channel_override: Some("nightly"),
        has: true,
    };

    const NIGHTLY_ALLOWED: Setup = Setup {
        config_dir: Some("allowed"),
        channel_override: Some("nightly"),
        has: true,
    };

    const NIGHTLY_FORBIDDEN: Setup = Setup {
        config_dir: Some("forbidden"),
        channel_override: Some("nightly"),
        has: false,
    };

    const STABLE: Setup = Setup {
        config_dir: None,
        channel_override: Some("stable"),
        has: false,
    };

    const BETA: Setup = Setup {
        config_dir: None,
        channel_override: Some("beta"),
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
        runtest(example.clone(), setup);
        clippy(example, setup);
    }
}

mod stable {
    use super::*;

    const STABLE: Setup = Setup {
        config_dir: None,
        channel_override: Some("stable"),
        has: true,
    };

    const BETA: Setup = Setup {
        config_dir: None,
        channel_override: Some("beta"),
        has: true,
    };

    const NIGHTLY: Setup = Setup {
        config_dir: None,
        channel_override: Some("nightly"),
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
        runtest(example.clone(), setup);
        clippy(example, setup);
    }
}

mod beta {
    use super::*;

    const STABLE: Setup = Setup {
        config_dir: None,
        channel_override: Some("stable"),
        has: false,
    };

    const BETA: Setup = Setup {
        config_dir: None,
        channel_override: Some("beta"),
        has: true,
    };

    const NIGHTLY: Setup = Setup {
        config_dir: None,
        channel_override: Some("nightly"),
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
    /// Runs the tests for each example under `examples/beta`
    ///
    /// All examples have a rust-toolchain.toml which specifies a nightly channel from before stabilisation.
    ///
    /// All examples have 2 subdirs `allowed` & `forbidden`, each containing a `.cargo/config.toml` which
    /// either specifically allows or forbids the feature. This supports cases where one feature depends
    /// on others also being enabled (e.g. unsized_const_params, try_trait_v2).
    fn examples(
        #[files("*")]
        #[dirs]
        #[base_dir = "examples/beta"]
        example: PathBuf,
        #[values(NIGHTLY, STABLE, BETA, PRE_STABILISATION, PRE_ALLOWED, PRE_FORBIDDEN)]
        setup: Setup,
    ) {
        runtest(example.clone(), setup);
        clippy(example, setup);
    }
}

fn runtest(example: PathBuf, setup: Setup) {
    let Setup {
        config_dir,
        channel_override,
        has,
    } = setup;

    let mut test = cargo_foo(&["test"], example, config_dir, channel_override);

    let output = test.output().unwrap();
    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if has {
        assert!(
            stdout.contains("has::"),
            "incorrect tests run: {status} {stdout} {stderr}"
        );
    } else {
        assert!(
            stdout.contains("has_not::"),
            "incorrect tests run: {status} {stdout} {stderr}"
        );
    };

    assert!(
        status.success(),
        "test execution failed with {status} {stdout} {stderr}"
    );
}

/// Run `clippy -- -D warnings` which has a tendency to fail more complex probes if they are
/// not written correctly.
fn clippy(example: PathBuf, setup: Setup) {
    let Setup {
        config_dir,
        channel_override,
        has,
    } = setup;

    let mut clippy = cargo_foo(
        &["clippy", "--message-format", "json", "--", "-D", "warnings"],
        example.clone(),
        config_dir,
        channel_override,
    );
    let output = clippy.output().unwrap();
    let status = output.status;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        status.success(),
        "clippy failed with {status} {stdout} {stderr}"
    );

    let compile_output = stdout
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .find(|line: &ClippyOutput| {
            line.reason == "build-script-executed"
                && line
                    .package_id
                    .contains(example.to_str().expect("valid example path"))
        })
        .expect("compiled example");

    let cfg_has_foo = compile_output
        .cfgs
        .expect("cfgs present")
        .into_iter()
        .find(|cfg| cfg.starts_with("has_"));

    if has {
        assert!(cfg_has_foo.is_some(), "has_... was not set by build script")
    } else {
        assert!(
            cfg_has_foo.is_none(),
            "has_... was incorrectly set by build script"
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ClippyOutput {
    reason: String,
    package_id: String,
    cfgs: Option<Vec<String>>,
}

fn cargo_foo(
    subcommand: &[&str],
    example: PathBuf,
    config_dir: Option<&'static str>,
    channel_override: Option<&'static str>,
) -> Command {
    let mut cargo_foo = Command::new("cargo");
    cargo_foo
        .args(subcommand)
        .current_dir(&example)
        .env("RUSTC_BOOTSTRAP", "0")
        // We need to read the rust-toolchain.toml ourselves and set RUSTUP_TOOLCHAIN
        // as cargo absolutely refuses to run on a different toolchain than the one
        // invoking the top level cargo test
        .env(
            "RUSTUP_TOOLCHAIN",
            match channel_override {
                Some(channel) => channel.to_string(),
                None => {
                    let rust_toolchain_toml = example.join("rust-toolchain.toml");
                    fs::read_to_string(rust_toolchain_toml)
                        .unwrap()
                        .parse::<Table>()
                        .unwrap()
                        .get("toolchain")
                        .map(|v| Table::try_from(v.clone()).unwrap())
                        .unwrap()
                        .get("channel")
                        .map(|v| v.as_str().unwrap().to_string())
                        .unwrap()
                }
            },
        );
    if let Some(config) = config_dir {
        cargo_foo.env("BUILD_SAFELY_CARGO_CONFIG_DIR", example.join(config));
    };
    cargo_foo
}
