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
    process::Command,
};

use tempfile::TempDir;

/// All the nightly feature examples to test
const EXAMPLES: &[(&str, &[&str])] = &[
    (
        "build_safely-adt_const_params_fixture",
        &["adt_const_params"],
    ),
    ("build_safely-bool_to_result_fixture", &["bool_to_result"]),
    ("build_safely-can_vector_fixture", &["can_vector"]),
    (
        "build_safely-doc_notable_trait_fixture",
        &["doc_notable_trait"],
    ),
    (
        "build_safely-iterator_try_collect_fixture",
        &["iterator_try_collect"],
    ),
    // Grouped example with multiple features
    (
        "build_safely-never_type_try_v2_residual_fixture",
        &["never_type", "try_trait_v2", "try_trait_v2_residual"],
    ),
    (
        "build_safely-proc_macro_diagnostic_fixture",
        &["proc_macro_diagnostic"],
    ),
    ("build_safely-strip_circumfix_fixture", &["strip_circumfix"]),
    (
        "build_safely-unsized_const_params_fixture",
        &["unsized_const_params"],
    ),
    (
        "build_safely-write_all_vectored_fixture",
        &["write_all_vectored"],
    ),
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
// TODO rename `package` to express actual meaning or remove if possible - is it "feature"?, it shouldn't even be required if only copying the one example to a tempdir
// TODO return Result<Output> directly
fn run_cargo_test(config_dir: &Path, package: &str) -> String {
    let output = Command::new("cargo")
        .arg("test")
        .arg("-p")
        .arg(package)
        .arg("--")
        .arg("--nocapture")
        //TODO: use correct variable BUILD_SAFELY_CARGO_CONFIG_DIR
        .env("CARGO_CONFIG_DIR", config_dir)
        .output()
        .expect("Failed to run cargo test");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    format!("{stdout}\n{stderr}")
}

/// Check if any "has" test ran in the output
fn has_test_ran(output: &str) -> bool {
    // TODO this looks really flaky - a single check should be sufficient if there is a good naming convention in place.
    output.contains("test has::")
        || output.contains("test has_never_type::")
        || output.contains("test has_try_trait_v2::")
        || output.contains("test has_try_trait_v2_residual::")
        // TODO this will pass for "test tests::has_not" - BUG
        || output.contains("test tests::has")
}

/// Check if any "has_not" test ran in the output
fn has_not_test_ran(output: &str) -> bool {
    //TODO Stick to a good naming convention that allows for a simple and secure check in this and has_test_ran
    output.contains("test has_not::") || output.contains("test tests::has_not")
}

/// Check if any test ran at all
fn any_test_ran(output: &str) -> bool {
    has_test_ran(output) || has_not_test_ran(output)
}


// TODO add rstest to the dev dependencies and read the top level doc comment from the src/lib.rs to understand how to use parametrization and fixtures
// TODO use crate rstest & parametrizse with #[case] instead of for loop and println!. Rstest will list each test as an independant test case with a clear result for each.
// TODO Test 1 & Test 2 This should be independent tests with good names. Add a 3rd test with _all_ features allowed (no config.toml)
// TODO make use of rstest fixtures to create tempdir with .cargo/config.toml
// TODO prefer a simple .unwrap() to check for errors - separating the tests and parametrizing them provides all the context needed, unwrap then outputs the error details.
// TODO prefer assert!(has_test_ran(...), custom message) to if else panic
#[test]
fn test_all_examples() {
    for (package_name, features) in EXAMPLES {
        println!("Testing package: {package_name}");

        // Test 1: Run with NO features allowed
        // This should run either "has_not" (feature unstable) or "has" (feature stabilized)
        println!("  Testing with NO features allowed...");
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        create_config(temp_dir.path(), &[])
            .expect(&format!("Failed to create config for {package_name}"));

        let output = run_cargo_test(temp_dir.path(), package_name);

        if !any_test_ran(&output) {
            panic!("No test ran for {package_name} with no features allowed.\n\nOutput:\n{output}");
        }

        // Exactly one of has or has_not should run, not both
        let has_ran = has_test_ran(&output);
        let has_not_ran = has_not_test_ran(&output);

        if has_ran && has_not_ran {
            panic!(
                "Both 'has' and 'has_not' tests ran for {package_name} with no features allowed.\n\nOutput:\n{output}"
            );
        }

        if has_ran {
            println!("    ✓ has test ran (feature is stabilized on this compiler)");
        } else {
            println!("    ✓ has_not test ran (feature is still unstable)");
        }

        // Clean up temp dir (automatically done when TempDir is dropped)
        drop(temp_dir);

        // Test 2: Run WITH the feature(s) allowed
        // This should ALWAYS run the "has" test(s)
        println!("  Testing with features allowed: {:?}...", features);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        create_config(temp_dir.path(), features)
            .expect(&format!("Failed to create config for {package_name}"));

        let output = run_cargo_test(temp_dir.path(), package_name);

        if !has_test_ran(&output) {
            panic!(
                "Expected 'has' test to run for {package_name} with features {:?} allowed.\n\nOutput:\n{output}",
                features
            );
        }

        // Make sure has_not did NOT run
        if has_not_test_ran(&output) {
            panic!(
                "Unexpected 'has_not' test ran for {package_name} with features {:?} allowed.\n\nOutput:\n{output}",
                features
            );
        }

        println!("    ✓ has test(s) ran (feature available)");

        // Clean up temp dir (automatically done when TempDir is dropped)
        drop(temp_dir);

        println!("  ✓ {package_name} passed all tests");
    }
}
