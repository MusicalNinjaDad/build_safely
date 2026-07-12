//! Integration tests for nightly features
//!
//! This test verifies that each example correctly:
//! 1. Runs the "has" test when the feature is available (in allow-features)
//! 2. Runs the "has_not" test when the feature is NOT available (empty allow-features)
//!
//! Note: Some features may be stabilized on the current nightly compiler.
//! In such cases, the "has" test will run even with empty allow-features.

use std::{
    fs::{self, File},
    io::Write,
    path::Path,
    process::Output,
};

use rstest::*;
use tempfile::TempDir;

/// All nightly features used by the examples
const ALL_FEATURES: &[&str] = &[
    "adt_const_params",
    "bool_to_result",
    "can_vector",
    "doc_notable_trait",
    "iterator_try_collect",
    "never_type",
    "try_trait_v2",
    "try_trait_v2_residual",
    "proc_macro_diagnostic",
    "strip_circumfix",
    "unsized_const_params",
    "write_all_vectored",
];

/// Create a .cargo/config.toml with the given allowed features in a directory
fn create_config(dir: &Path, allowed_features: &[&str]) -> std::io::Result<()> {
    let cargo_dir = dir.join(".cargo");
    fs::create_dir_all(&cargo_dir)?;

    let config_path = cargo_dir.join("config.toml");
    let mut file = File::create(&config_path)?;

    if allowed_features.is_empty() {
        writeln!(file, "unstable.allow-features = []")?;
    } else {
        writeln!(file, "unstable.allow-features = [")?;
        for feature in allowed_features {
            writeln!(file, "\"{feature}\",")?;
        }
        writeln!(file, "]")?;
    }

    Ok(())
}

/// Run cargo test for a specific package from a directory and return the output
fn run_cargo_test(config_dir: &Path, package: &str) -> std::io::Result<Output> {
    let output = std::process::Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg(package)
        .arg("--")
        .arg("--nocapture")
        .env("BUILD_SAFELY_CARGO_CONFIG_DIR", config_dir)
        .output()?;
    Ok(output)
}

/// Check if any "has" test ran in the output
fn has_test_ran(output: &Output) -> bool {
    let output_str = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    // Check for consistent test naming patterns
    // Be careful to avoid matching "has_not"
    // Patterns: "test has::", "test has_<name>", "test tests::has "
    output_str.contains("test has::")
        || output_str.contains("test tests::has_never_type")
        || output_str.contains("test tests::has_try_trait_v2")
        || output_str.contains("test tests::has_try_trait_v2_residual")
        || output_str.contains("test tests::has ")
}

/// Check if any "has_not" test ran in the output
fn has_not_test_ran(output: &Output) -> bool {
    let output_str = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    output_str.contains("test has_not::") || output_str.contains("test tests::has_not")
}

/// Check if any test ran at all
fn any_test_ran(output: &Output) -> bool {
    has_test_ran(output) || has_not_test_ran(output)
}

// Fixture to create a temp directory with .cargo/config.toml
#[fixture]
fn config_dir(#[default(Vec::new())] allowed_features: Vec<&'static str>) -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    create_config(temp_dir.path(), &allowed_features).unwrap();
    temp_dir
}

// Test 1: Run with NO features allowed
// This should run either "has_not" (feature unstable) or "has" (feature stabilized)
// Exactly one of has or has_not should run, not both
#[rstest]
#[case::adt_const_params("build_safely-adt_const_params_fixture", vec!["adt_const_params"])]
#[case::bool_to_result("build_safely-bool_to_result_fixture", vec!["bool_to_result"])]
#[case::can_vector("build_safely-can_vector_fixture", vec!["can_vector"])]
#[case::doc_notable_trait("build_safely-doc_notable_trait_fixture", vec!["doc_notable_trait"])]
#[case::iterator_try_collect("build_safely-iterator_try_collect_fixture", vec!["iterator_try_collect"])]
#[case::never_type_try_v2_residual("build_safely-never_type_try_v2_residual_fixture", vec!["never_type", "try_trait_v2", "try_trait_v2_residual"])]
#[case::proc_macro_diagnostic("build_safely-proc_macro_diagnostic_fixture", vec!["proc_macro_diagnostic"])]
#[case::strip_circumfix("build_safely-strip_circumfix_fixture", vec!["strip_circumfix"])]
#[case::unsized_const_params("build_safely-unsized_const_params_fixture", vec!["unsized_const_params"])]
#[case::write_all_vectored("build_safely-write_all_vectored_fixture", vec!["write_all_vectored"])]
fn test_no_features_allowed_exactly_one_runs(
    #[case] package_name: &'static str,
    #[case] _features: Vec<&'static str>,
    config_dir: TempDir,
) {
    let output = run_cargo_test(config_dir.path(), package_name).unwrap();

    assert!(
        any_test_ran(&output),
        "No test ran for {package_name} with no features allowed.\n\nOutput:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let has_ran = has_test_ran(&output);
    let has_not_ran = has_not_test_ran(&output);

    assert!(
        has_ran ^ has_not_ran,
        "Exactly one of 'has' or 'has_not' should run for {package_name} with no features allowed. has={}, has_not={}\n\nOutput:\n{}{}",
        has_ran,
        has_not_ran,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

// Test 2: Run WITH features allowed
// This should ALWAYS run the "has" test(s)
// has_not should NOT run
#[rstest]
#[case::adt_const_params("build_safely-adt_const_params_fixture", vec!["adt_const_params"])]
#[case::bool_to_result("build_safely-bool_to_result_fixture", vec!["bool_to_result"])]
#[case::can_vector("build_safely-can_vector_fixture", vec!["can_vector"])]
#[case::doc_notable_trait("build_safely-doc_notable_trait_fixture", vec!["doc_notable_trait"])]
#[case::iterator_try_collect("build_safely-iterator_try_collect_fixture", vec!["iterator_try_collect"])]
#[case::never_type_try_v2_residual("build_safely-never_type_try_v2_residual_fixture", vec!["never_type", "try_trait_v2", "try_trait_v2_residual"])]
#[case::proc_macro_diagnostic("build_safely-proc_macro_diagnostic_fixture", vec!["proc_macro_diagnostic"])]
#[case::strip_circumfix("build_safely-strip_circumfix_fixture", vec!["strip_circumfix"])]
#[case::unsized_const_params("build_safely-unsized_const_params_fixture", vec!["unsized_const_params"])]
#[case::write_all_vectored("build_safely-write_all_vectored_fixture", vec!["write_all_vectored"])]
fn test_with_features_allowed_has_runs(
    #[case] package_name: &'static str,
    #[case] features: Vec<&'static str>,
    config_dir: TempDir,
) {
    // Recreate config with the specific features for this package
    create_config(config_dir.path(), &features).unwrap();

    let output = run_cargo_test(config_dir.path(), package_name).unwrap();

    assert!(
        has_test_ran(&output),
        "Expected 'has' test to run for {package_name} with features {:?} allowed.\n\nOutput:\n{}{}",
        features,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        !has_not_test_ran(&output),
        "Unexpected 'has_not' test ran for {package_name} with features {:?} allowed.\n\nOutput:\n{}{}",
        features,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

// Test 3: Run with ALL features allowed (all features in allow-features)
// This should ALWAYS run the "has" test(s) for all examples
#[rstest]
#[case::adt_const_params("build_safely-adt_const_params_fixture")]
#[case::bool_to_result("build_safely-bool_to_result_fixture")]
#[case::can_vector("build_safely-can_vector_fixture")]
#[case::doc_notable_trait("build_safely-doc_notable_trait_fixture")]
#[case::iterator_try_collect("build_safely-iterator_try_collect_fixture")]
#[case::never_type_try_v2_residual("build_safely-never_type_try_v2_residual_fixture")]
#[case::proc_macro_diagnostic("build_safely-proc_macro_diagnostic_fixture")]
#[case::strip_circumfix("build_safely-strip_circumfix_fixture")]
#[case::unsized_const_params("build_safely-unsized_const_params_fixture")]
#[case::write_all_vectored("build_safely-write_all_vectored_fixture")]
fn test_all_features_allowed_has_runs(#[case] package_name: &'static str, config_dir: TempDir) {
    // Recreate config with all features allowed
    create_config(config_dir.path(), ALL_FEATURES).unwrap();

    let output = run_cargo_test(config_dir.path(), package_name).unwrap();

    assert!(
        has_test_ran(&output),
        "Expected 'has' test to run for {package_name} with all features allowed.\n\nOutput:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        !has_not_test_ran(&output),
        "Unexpected 'has_not' test ran for {package_name} with all features allowed.\n\nOutput:\n{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
