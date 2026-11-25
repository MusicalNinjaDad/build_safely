use std::{
    fmt::Debug,
    path::Path,
    process::{Command, Output},
};

use crate::{BuildError, Result, get_var};

use super::UnstableFeature;

/// Check whether cargo will accept unstable flags. You probably never need to run this
/// yourself and should prefer to simply call [`cargo_allowed_features`].
pub fn cargo_unstable() -> Result<bool> {
    let mut cmd = Command::new(get_var("CARGO")?);
    cmd.args([
        "-Zunstable-options",
        "--config",
        "unstable.allow-features=[\"unstable-options\"]",
        "help",
    ]);
    let output = cmd
        .output()
        .map_err(|err| BuildError::Other(err.to_string()))?;
    Ok(output.status.success())
}

fn cargo_config<P: AsRef<Path>>(
    current_dir: &Option<P>,
    added_unstable_options: bool,
) -> Result<Output> {
    let mut cargo_config_get = Command::new(get_var("CARGO")?);
    if let Some(dir) = &current_dir {
        cargo_config_get.current_dir(dir);
    }
    cargo_config_get.arg("-Zunstable-options");
    if added_unstable_options {
        cargo_config_get.args(["--config", "unstable.allow-features=[\"unstable-options\"]"]);
    }
    cargo_config_get.args(["config", "get"]);

    // show in `cargo build -vv`
    dbg!(&cargo_config_get);

    cargo_config_get
        .output()
        .map_err(|err| BuildError::Other(err.to_string()))
}

/// Identify which experimental features are allowed for this build.
///
/// This works fine on any channel and respects whitelists (`unstable.allowed-features`) in all
/// relevant cargo config.toml files.
///
/// ## Note to downstream crates
///
/// Due to limitations in the information provided by cargo:
///
/// - This will obtain config.toml files based upon `OUT_DIR`. If this is not under the project
///   root, you can override by providing an alternative path via the environment variable
///   `BUILD_SAFELY_CARGO_CONFIG_DIR`. See cargo's documentation on config file hierarchical structure
///   for more details.
/// - This will not respect additional entries passed at the command line via
///   `cargo --config unstable.allow-features=[...]`
pub fn cargo_allowed_features() -> Result<AllowedFeatures> {
    println!("cargo::rerun-if-env-changed=BUILD_SAFELY_CARGO_CONFIG_DIR");
    let cwd = std::env::var("BUILD_SAFELY_CARGO_CONFIG_DIR")
        .or_else(|_| std::env::var("OUT_DIR"))
        .ok();
    _cargo_allowed_features(cwd)
}

fn _cargo_allowed_features<P: AsRef<Path> + Debug>(
    current_dir: Option<P>,
) -> Result<AllowedFeatures> {
    if !cargo_unstable()? {
        // show in `cargo build -vv`
        dbg!("cargo won't accept `-Z` - so we're on a not-unstable toolchain");

        let allowed_features = AllowedFeatures(_AllowedFeatures::None);

        // show in `cargo build -vv`
        dbg!(&allowed_features);
        return Ok(allowed_features);
    }

    let mut added_unstable_options = false;
    let mut output = cargo_config(&current_dir, added_unstable_options)?;

    if !output.status.success() {
        // Maybe there is a restricted list which doesn't include unstable-options
        added_unstable_options = true;
        output = cargo_config(&current_dir, added_unstable_options)?;

        if !output.status.success() {
            // Nope something else went wrong!
            return Err(BuildError::Other(format!(
                "cargo config failed with error {code}: {stderr}",
                code = output.status,
                stderr = String::from_utf8_lossy(&output.stderr)
            )));
        }
    };

    let cargo_config = String::from_utf8_lossy(&output.stdout);

    let allowed_features = match cargo_config
        .lines()
        .find(|line| line.starts_with("unstable.allow-features"))
    {
        None => AllowedFeatures(_AllowedFeatures::All),
        Some(features) => {
            // default output format is toml
            let features: Vec<_> = features
                .strip_prefix("unstable.allow-features = [")
                .ok_or_else(|| {
                    BuildError::Other(format!(
                        "invalid cargo config output: {}",
                        String::from_utf8_lossy(&output.stdout)
                    ))
                })?
                .strip_suffix("]")
                .ok_or_else(|| {
                    BuildError::Other(format!(
                        "invalid cargo config output: {}",
                        String::from_utf8_lossy(&output.stdout)
                    ))
                })?
                .replace("\"", "")
                .split(",")
                .map(str::trim)
                .filter(|feature| !added_unstable_options || *feature != "unstable-options")
                .map(UnstableFeature::from)
                .collect();
            if features.is_empty() {
                AllowedFeatures(_AllowedFeatures::None)
            } else {
                AllowedFeatures(_AllowedFeatures::Some(features))
            }
        }
    };

    // show in `cargo build -vv`
    dbg!(&allowed_features);

    Ok(allowed_features)
}

/// The set of allowed experimental features for the current build. The only way to create this
/// is via a call to [cargo_allowed_features] - this is deliberate, to ensure that people who have
/// decided to restrict the experimental features they use to a whitelist are respected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllowedFeatures(_AllowedFeatures);

impl AllowedFeatures {
    /// Not public as this doesn't consider any restrictions made via `RUSTFLAGS`, those
    /// features will be disabled for all calls to rustc when running probes.
    pub(super) fn includes(&self, feature: &UnstableFeature) -> bool {
        match &self.0 {
            _AllowedFeatures::None => false,
            _AllowedFeatures::All => true,
            _AllowedFeatures::Some(features) => features.iter().find(|f| *f == feature).is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// private to make it impossible to manually construct an [AllowedFeatures] from outside this crate
enum _AllowedFeatures {
    None,
    All,
    Some(Vec<UnstableFeature>),
}

#[cfg(test)]
mod tests {
    use std::{
        assert_matches,
        fs::{self, File},
        io::Write,
    };

    use strum::IntoEnumIterator;
    use tempfile::TempDir;

    use super::super::probes;
    use super::UnstableFeature::*;
    use super::*;

    #[test]
    fn no_config_toml() {
        let tmp = TempDir::new().expect("tempdir");
        let allowed = _cargo_allowed_features(Some(&tmp));
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::All)));
            assert!(allowed.unwrap().includes(&try_trait_v2));
        } else {
            assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::None)));
            assert!(!allowed.unwrap().includes(&try_trait_v2));
        }
    }

    #[test]
    fn allowed_features() {
        let tmp = TempDir::new().expect("tempdir");
        let config_location = tmp.path().join(".cargo");
        fs::create_dir(&config_location).expect(".cargo created");
        dbg!(&config_location);
        let mut config =
            File::create_new(config_location.join("config.toml")).expect("create config.toml");
        writeln!(
            config,
            "unstable.allow-features = [\"try_trait_v2\", \"unstable-options\"]"
        )
        .expect("added to config");

        let allowed = _cargo_allowed_features(Some(&tmp)).unwrap();
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(
                allowed,
                AllowedFeatures(_AllowedFeatures::Some(ref features))
                if features == &vec![try_trait_v2, OtherFeature("unstable-options".to_string())]
            );
            assert!(allowed.includes(&try_trait_v2));
            assert!(allowed.includes(&OtherFeature("unstable-options".to_string())));
        } else {
            assert_matches!(allowed, AllowedFeatures(_AllowedFeatures::None));
            assert!(!allowed.includes(&try_trait_v2));
            assert!(!allowed.includes(&OtherFeature("unstable-options".to_string())));
        }
    }

    #[test]
    fn allowed_features_no_unstable_options() {
        let tmp = TempDir::new().expect("tempdir");
        let config_location = tmp.path().join(".cargo");
        fs::create_dir(&config_location).expect(".cargo created");
        dbg!(&config_location);
        let mut config =
            File::create_new(config_location.join("config.toml")).expect("create config.toml");
        writeln!(config, "unstable.allow-features = [\"try_trait_v2\"]").expect("added to config");

        let allowed = _cargo_allowed_features(Some(&tmp)).unwrap();
        if cargo_unstable().expect("cargo_unstable") {
            assert_matches!(
                allowed,
                AllowedFeatures(_AllowedFeatures::Some(ref features))
                if features == &vec![try_trait_v2]
            );
            assert!(allowed.includes(&try_trait_v2));
        } else {
            assert_matches!(allowed, AllowedFeatures(_AllowedFeatures::None));
            assert!(!allowed.includes(&try_trait_v2));
        }
    }

    #[test]
    fn all_forbidden() {
        let tmp = TempDir::new().expect("tempdir");
        let config_location = tmp.path().join(".cargo");
        fs::create_dir(&config_location).expect(".cargo created");
        dbg!(&config_location);
        let mut config =
            File::create_new(config_location.join("config.toml")).expect("create config.toml");
        writeln!(config, "unstable.allow-features = []").expect("added to config");

        let allowed = _cargo_allowed_features(Some(&tmp));
        assert_matches!(allowed, Ok(AllowedFeatures(_AllowedFeatures::None)));
    }

    #[test]
    fn make_assert_matches_probe() {
        let expected = r#"
#![allow(stable_features)]
#![feature(assert_matches)]
#![allow(unused)]

use std::assert_matches;
"#;
        let probe = r#"
use std::assert_matches;
"#;

        assert_eq!(probes::make_probe(&assert_matches, true, probe), expected);
    }

    #[test]
    fn unstable_feature_display() {
        assert_eq!(
            "foo",
            format!("{}", UnstableFeature::OtherFeature("foo".to_string()))
        );
        assert_eq!("try_trait_v2", format!("{}", UnstableFeature::try_trait_v2))
    }

    #[test]
    fn from_str_complete() {
        for feature in UnstableFeature::iter() {
            assert_eq!(UnstableFeature::from(feature.to_string().as_str()), feature);
        }
    }
}
