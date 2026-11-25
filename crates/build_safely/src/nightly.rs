#![expect(clippy::test_attr_in_doctest)]
//! Checking for experimental or stabilised features is prone to subtle errors which create issues
//! for downstream users and verbose when done properly. This provides extensions to the amazing
//! [autocfg::AutoCfg] (re-exported via our prelude to make your life easier) to safely identify the
//! availability of nightly features & handle the future stabilisation process without additional
//! effort on your part. All while respecting any `allow-features` whitelists.
//!
//! For a list of known features with dedicated probes see [UnstableFeature]
//!
//! # Usage
//!
//! ## In `build.rs`
//!
//! ```rust, no_run
//! use build_safely::prelude::*;
//!
//! fn main() -> Result<()> {
//!     // get a new AutoCfg or provide a valuable error
//!     // rather than panicing
//!     //
//!     // We require `mut` as we set edition 2024 by default, unless you have
//!     // explicitly called `ac.set_edition(...)`
//!     let mut ac = AutoCfg::new()?;
//!
//!     // check to see if the downstream crate has defined
//!     // `unstable.allow-features` in `.cargo/config.toml`.
//!     // It is mandatory to perform this check and pass the
//!     // result to any calls to `emit_unstable_feature`
//!     let allowed_features = cargo_allowed_features()?;
//!
//!     // We want to make use of `assert_matches` if it is available
//!     ac.emit_unstable_feature(assert_matches, &allowed_features);
//!     //                       ^^^^^^^^^^^^^^ - enum variant to avoid typos
//!
//!     Ok(())
//! }
//! ```
//!
//! ## In `lib.rs` / `main.rs`
//!
//! ```rust
//! // only enable unstable feature if it is available and has not yet been stabilised
//! #![cfg_attr(unstable_assert_matches, feature(assert_matches))]
//!
//! #[cfg(test)]
//! // Do these tests if `assert_matches` is available
//! #[cfg(has_assert_matches)]
//! mod tests {
//!     // `assert_matches` was moved in early 2026 before stabilisation
//!     #[cfg(assert_matches_location = "root")]
//!     use std::assert_matches;
//!
//!     // in earlier nightly compilers `assert_matches` was in a separate module
//!     #[cfg(assert_matches_location = "module")]
//!     use std::assert_matches::assert_matches;
//!
//!     #[test]
//!     fn has() {
//!         assert_matches!(Some(5), Some(n) if n == 5);
//!     }
//! }
//!
//! #[cfg(test)]
//! // Do these tests if `assert_matches` is not available
//! #[cfg(not(has_assert_matches))]
//! mod tests {
//!     #[test]
//!     fn has_not() {
//!         assert_eq!(Some(5), Some(5));
//!     }
//! }
//! ```
//!
//! # Note to downstream crates
//!
//! If you (transiently) depend on a crate which uses `build_safely` and have implemented a
//! whitelist of `allowed-features`.
//!
//! Due to limitations in the information provided by cargo:
//!
//! - This will obtain config.toml files based upon `OUT_DIR`. If this is not under the project
//!   root, you can override by providing an alternative path via the environment variable
//!   `BUILD_SAFELY_CARGO_CONFIG_DIR`. See cargo's documentation on config file hierarchical structure
//!   for more details.
//! - This will not respect additional entries passed at the command line via
//!   `cargo --config unstable.allow-features=[...]`

use std::fmt::Debug;

/// re-exported from autocfg
///
pub use autocfg::AutoCfg;
use strum::Display;
#[cfg(test)]
use strum::EnumIter;

mod probes;
use probes::{has, make_probe, unstable};

mod allowed_features;
#[doc(inline)]
pub use allowed_features::{AllowedFeatures, cargo_allowed_features, cargo_unstable};

mod nightly_trait;
#[doc(inline)]
pub use nightly_trait::Nightly;

/// Known features with `unstable_...` & dedicated probes for `has_...`.
///
/// If the feature you want is not in this list you can use `Other` to get `unstable_...`
/// but please also raise a PR (or open an issue) to add a custom probe for `has_...`.
#[allow(non_camel_case_types, reason = "shadowing feature naming")]
#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[cfg_attr(test, derive(EnumIter))]
#[non_exhaustive]
pub enum UnstableFeature {
    /// ## Provides cfg flags for feature [`adt_const_params`](https://github.com/rust-lang/rust/issues/95174)
    /// - `#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]`
    /// - `#[cfg(has_adt_const_params)]`
    adt_const_params,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_assert_matches, feature(assert_matches))]`
    /// - `#[cfg(has_assert_matches)]`
    /// - ```rust, ignore
    ///   #[cfg(assert_matches_location = "root")]
    ///   use std::assert_matches;
    ///   ```
    /// - ```rust, ignore
    ///   #[cfg(assert_matches_location = "module")]
    ///   use std::assert_matches::assert_matches;
    ///   ```
    assert_matches,
    /// ## Provides cfg flags for feature [`bool_to_result`](https://github.com/rust-lang/rust/issues/142748)
    /// - `#![cfg_attr(unstable_bool_to_result, feature(bool_to_result))]`
    /// - `#[cfg(has_bool_to_result)]`
    /// - this gates [`bool::ok_or`] & [`bool::ok_or_else`]
    /// - **Stable since 1.98.0**
    bool_to_result,
    /// ## Provides cfg flags for feature [`can_vector`](https://github.com/rust-lang/rust/issues/69941)
    /// - `#![cfg_attr(unstable_can_vector, feature(can_vector))]`
    /// - `#[cfg(has_can_vector)]`
    /// - this gates [`std::io::Read::is_read_vectored`] & [`std::io::Write::is_write_vectored`]
    can_vector,
    /// ## Provides cfg flags for feature [`const_bool`](https://github.com/rust-lang/rust/issues/151531)
    /// - `#![cfg_attr(unstable_const_bool, feature(const_bool))]`
    /// - `#[cfg(has_const_bool)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_ops` requires `const_trait_impl` to be enabled
    const_bool,
    /// ## Provides cfg flags for feature [`const_cmp`](https://github.com/rust-lang/rust/issues/143800)
    /// - `#![cfg_attr(unstable_const_cmp, feature(const_cmp))]`
    /// - `#[cfg(has_const_cmp)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_ops` requires `const_trait_impl` to be enabled
    const_cmp,
    /// ## Provides cfg flags for feature [`const_convert`](https://github.com/rust-lang/rust/issues/143773)
    /// - `#![cfg_attr(unstable_const_convert, feature(const_convert))]`
    /// - `#[cfg(has_const_convert)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_ops` requires `const_trait_impl` to be enabled
    const_convert,
    /// ## Provides cfg flags for feature [`const_destruct`](https://github.com/rust-lang/rust/issues/143773)
    /// - `#![cfg_attr(unstable_const_destruct, feature(const_destruct))]`
    /// - `#[cfg(has_const_destruct)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_ops` requires `const_trait_impl` to be enabled
    const_destruct,
    /// ## Provides cfg flags for feature [`const_ops`](https://github.com/rust-lang/rust/issues/133214)
    /// - `#![cfg_attr(unstable_const_ops, feature(const_ops))]`
    /// - `#[cfg(has_const_ops)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_ops` requires `const_trait_impl` to be enabled
    const_ops,
    /// ## Provides cfg flags for feature [`const_option_ops`](https://github.com/rust-lang/rust/issues/133214)
    /// - `#![cfg_attr(unstable_const_option_ops, feature(const_option_ops))]`
    /// - `#[cfg(has_const_option_ops)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_option_ops` requires `const_trait_impl` to be enabled
    const_option_ops,
    /// ## Provides cfg flags for feature [`const_result_trait_fn`](https://github.com/rust-lang/rust/issues/144211)
    /// - `#![cfg_attr(unstable_const_result_trait_fn, feature(const_result_trait_fn))]`
    /// - `#[cfg(has_const_result_trait_fn)]`
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - Note: `const_result_trait_fn` requires `const_trait_impl` to be enabled
    const_result_trait_fn,
    /// ## Provides cfg flags for feature [`const_trait_impl`](https://github.com/rust-lang/rust/issues/143956)
    /// - `#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]`
    /// - `#[cfg(has_const_trait_impl)]`
    const_trait_impl,
    /// ## Provides cfg flags for feature [`default_field_values`](https://github.com/rust-lang/rust/issues/132162)
    /// - `#![cfg_attr(unstable_default_field_values, feature(default_field_values))]`
    /// - `#[cfg(has_default_field_values)]`
    default_field_values,
    /// ## Provides cfg flags for feature [`doc_notable_trait`](https://github.com/rust-lang/rust/issues/45040)
    /// - `#![cfg_attr(unstable_doc_notable_trait, feature(doc_notable_trait))]`
    /// - `#[cfg(has_doc_notable_trait)]`
    doc_notable_trait,
    /// ## Provides cfg flags for feature [`exact_div`](https://github.com/rust-lang/rust/issues/139911)
    /// - `#![cfg_attr(unstable_exact_div, feature(exact_div))]`
    /// - `#[cfg(exact_div)]`
    exact_div,
    /// ## Provides cfg flags for feature [`exact_size_is_empty`](https://github.com/rust-lang/rust/issues/35428)
    /// - `#![cfg_attr(unstable_exact_size_is_empty, feature(exact_size_is_empty))]`
    /// - `#[cfg(has_exact_size_is_empty)]`
    exact_size_is_empty,
    /// ## Provides cfg flags for feature [`integer_cast_extras`](https://github.com/rust-lang/rust/issues/154650)
    /// - `#![cfg_attr(unstable_integer_cast_extras, feature(integer_cast_extras))]`
    /// - `#[cfg(has_integer_cast_extras)]`
    /// - `#![cfg_attr(unstable_integer_casts, feature(integer_casts))]`
    /// - Note: `integer_cast_extras` requires `integer_casts` to be enabled
    integer_cast_extras,
    /// ## Provides cfg flags for feature [`integer_casts`](https://github.com/rust-lang/rust/issues/157388)
    /// - `#![cfg_attr(unstable_integer_casts, feature(integer_casts))]`
    /// - `#[cfg(has_integer_casts)]`
    integer_casts,
    /// ## Provides cfg flags for feature [`iter_array_chunks`](https://github.com/rust-lang/rust/issues/100450)
    /// - `#![cfg_attr(unstable_iter_array_chunks, feature(iter_array_chunks))]`
    /// - `#[cfg(has_iter_array_chunks)]`
    iter_array_chunks,
    /// ## Provides cfg flags for feature [`iter_next_chunk`](https://github.com/rust-lang/rust/issues/98326)
    /// - `#![cfg_attr(unstable_iter_next_chunk, feature(iter_next_chunk))]`
    /// - `#[cfg(has_iter_next_chunk)]`
    iter_next_chunk,
    /// ## Provides cfg flags for feature [`iter_map_windows`](https://github.com/rust-lang/rust/issues/87155)
    /// - `#![cfg_attr(unstable_iter_map_windows, feature(iter_map_windows))]`
    /// - `#[cfg(has_iter_map_windows)]`
    iter_map_windows,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_iterator_try_collect, feature(iterator_try_collect))]`
    /// - `#[cfg(has_iterator_try_collect)]`
    iterator_try_collect,
    /// ## Provides cfg flags for feature [`negative_impls`](https://github.com/rust-lang/rust/issues/68318)
    /// - `#![cfg_attr(unstable_negative_impls, feature(negative_impls))]`
    /// - `#[cfg(has_negative_impls)]`
    negative_impls,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_never_type, feature(never_type))]`
    /// - `#[cfg(has_never_type)]`
    never_type,
    /// ## Provides cfg flags for feature [`os_str_split_at`](https://github.com/rust-lang/rust/issues/156199)
    /// - `#![cfg_attr(unstable_os_str_split_at, feature(os_str_split_at))]`
    /// - `#[cfg(has_os_str_split_at)]`
    os_str_split_at,
    /// ## Provides cfg flags for feature [`path_absolute_method`](https://github.com/rust-lang/rust/issues/153328)
    /// - `#![cfg_attr(unstable_path_absolute_method, feature(path_absolute_method))]`
    /// - `#[cfg(has_path_absolute_method)]`
    path_absolute_method,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_proc_macro_diagnostic, feature(proc_macro_diagnostic))]`
    /// - `#[cfg(has_proc_macro_diagnostic)]`
    proc_macro_diagnostic,
    /// ## Provides cfg flags for feature [`strip_circumfix`](https://github.com/rust-lang/rust/issues/147946)
    /// - `#![cfg_attr(unstable_strip_circumfix, feature(strip_circumfix))]`
    /// - `#[cfg(has_strip_circumfix)]`
    strip_circumfix,
    /// ## Provides cfg flags for feature [`try_blocks`](https://github.com/rust-lang/rust/issues/154391)
    /// - `#![cfg_attr(unstable_try_blocks, feature(try_blocks))]`
    /// - `#[cfg(has_try_blocks)]`
    try_blocks,
    /// ## Provides cfg flags for feature [`try_blocks_heterogeneous`](https://github.com/rust-lang/rust/issues/149488)
    /// - `#![cfg_attr(unstable_try_blocks_heterogeneous, feature(try_blocks_heterogeneous))]`
    /// - `#[cfg(has_try_blocks_heterogeneous)]`
    try_blocks_heterogeneous,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_try_trait_v2, feature(try_trait_v2))]`
    /// - `#[cfg(has_try_trait_v2)]`
    try_trait_v2,
    /// ## Provides cfg flags:
    /// - `#![cfg_attr(unstable_try_trait_v2_residual, feature(try_trait_v2_residual))]`
    /// - `#[cfg(has_try_trait_v2_residual)]`
    try_trait_v2_residual,
    /// ## Provides cfg flags for feature [`unsized_const_params`](https://github.com/rust-lang/rust/issues/128028)
    /// - `#![cfg_attr(unstable_unsized_const_params, feature(unsized_const_params))]`
    /// - `#[cfg(has_unsized_const_params)]`
    /// - `#![cfg_attr(unstable_adt_const_params, feature(adt_const_params))]`
    /// - Note: `unsized_const_params` requires `adt_const_params` to be enabled
    unsized_const_params,
    /// ## Provides cfg flags for feature [`write_all_vectored`](https://github.com/rust-lang/rust/issues/70436)
    /// - `#![cfg_attr(unstable_write_all_vectored, feature(write_all_vectored))]`
    /// - `#[cfg(has_write_all_vectored)]`
    /// - this gates [`std::io::Write::write_all_vectored`]
    write_all_vectored,
    /// only provides `unstable_...` - please raise a PR to add a custom probe for `has_...`
    #[strum(to_string = "{0}")]
    OtherFeature(String),
}

impl UnstableFeature {
    /// This is not pub or trait From/FromStr to avoid risk of user typos leading to `OtherFeature`
    ///
    /// We use it solely when processing cargo allowed features. Hard-to-find issues with example
    /// tests not working for specific cases are likely due to a missing / incorrect mapping here.
    fn from(feature: &str) -> Self {
        match feature {
            "adt_const_params" => Self::adt_const_params,
            "assert_matches" => Self::assert_matches,
            "bool_to_result" => Self::bool_to_result,
            "can_vector" => Self::can_vector,
            "const_bool" => Self::const_bool,
            "const_cmp" => Self::const_cmp,
            "const_convert" => Self::const_convert,
            "const_destruct" => Self::const_destruct,
            "const_ops" => Self::const_ops,
            "const_option_ops" => Self::const_option_ops,
            "const_result_trait_fn" => Self::const_result_trait_fn,
            "const_trait_impl" => Self::const_trait_impl,
            "default_field_values" => Self::default_field_values,
            "doc_notable_trait" => Self::doc_notable_trait,
            "exact_div" => Self::exact_div,
            "exact_size_is_empty" => Self::exact_size_is_empty,
            "integer_cast_extras" => Self::integer_cast_extras,
            "integer_casts" => Self::integer_casts,
            "iter_array_chunks" => Self::iter_array_chunks,
            "iter_next_chunk" => Self::iter_next_chunk,
            "iter_map_windows" => Self::iter_map_windows,
            "iterator_try_collect" => Self::iterator_try_collect,
            "negative_impls" => Self::negative_impls,
            "never_type" => Self::never_type,
            "os_str_split_at" => Self::os_str_split_at,
            "path_absolute_method" => Self::path_absolute_method,
            "proc_macro_diagnostic" => Self::proc_macro_diagnostic,
            "strip_circumfix" => Self::strip_circumfix,
            "try_blocks" => Self::try_blocks,
            "try_blocks_heterogeneous" => Self::try_blocks_heterogeneous,
            "try_trait_v2" => Self::try_trait_v2,
            "try_trait_v2_residual" => Self::try_trait_v2_residual,
            "unsized_const_params" => Self::unsized_const_params,
            "write_all_vectored" => Self::write_all_vectored,
            _ => Self::OtherFeature(feature.to_string()),
        }
    }
}

impl Nightly for AutoCfg {
    fn emit_unstable_feature(
        &mut self,
        feature: UnstableFeature,
        allowed_features: &AllowedFeatures,
    ) -> bool {
        // show in `cargo build -vv`
        dbg!(&feature);

        if self.edition().is_none() {
            self.set_edition(Some("2024".to_string()));
        }

        let ac = self;

        let allowed = allowed_features.includes(&feature);
        match feature {
            UnstableFeature::adt_const_params => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::adt_const_params::AVAILABLE)
            }
            UnstableFeature::assert_matches => {
                unstable(ac, &feature, allowed, None);
                autocfg::emit_possibility("assert_matches_location, values(\"root\", \"module\")");
                if ac
                    .probe_raw(&make_probe(&feature, allowed, probes::assert_matches::ROOT))
                    .is_ok()
                {
                    autocfg::emit("assert_matches_location=\"root\"");
                } else if allowed && ac.probe_raw(probes::assert_matches::MODULE).is_ok() {
                    //    ^^^^^^^ assert_matches was stabilised in root
                    autocfg::emit("assert_matches_location=\"module\"");
                }
                has(ac, &feature, allowed, probes::assert_matches::AVAILABLE)
            }
            UnstableFeature::bool_to_result => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::bool_to_result::AVAILABLE)
            }
            UnstableFeature::can_vector => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::can_vector::AVAILABLE)
            }
            UnstableFeature::const_bool => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(ac, &feature, allowed, probes::const_bool::AVAILABLE)
            }
            UnstableFeature::const_cmp => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(ac, &feature, allowed, probes::const_cmp::AVAILABLE)
            }
            UnstableFeature::const_convert => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(ac, &feature, allowed, probes::const_convert::AVAILABLE)
            }
            UnstableFeature::const_destruct => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(ac, &feature, allowed, probes::const_destruct::AVAILABLE)
            }
            UnstableFeature::const_ops => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(ac, &feature, allowed, probes::const_ops::AVAILABLE)
            }
            UnstableFeature::const_option_ops => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(ac, &feature, allowed, probes::const_option_ops::AVAILABLE)
            }
            UnstableFeature::const_result_trait_fn => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::const_trait_impl,
                    allowed_features.includes(&UnstableFeature::const_trait_impl),
                    None,
                ) {
                    Some("#![feature(const_trait_impl)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::const_result_trait_fn::AVAILABLE,
                )
            }
            UnstableFeature::const_trait_impl => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::const_trait_impl::AVAILABLE)
            }
            UnstableFeature::default_field_values => {
                unstable(ac, &feature, allowed, None);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::default_field_values::AVAILABLE,
                )
            }
            UnstableFeature::doc_notable_trait => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::doc_notable_trait::AVAILABLE)
            }
            UnstableFeature::exact_div => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::exact_div::AVAILABLE)
            }
            UnstableFeature::exact_size_is_empty => {
                unstable(ac, &feature, allowed, None);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::exact_size_is_empty::AVAILABLE,
                )
            }
            UnstableFeature::integer_cast_extras => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::integer_casts,
                    allowed_features.includes(&UnstableFeature::integer_casts),
                    None,
                ) {
                    Some("#![feature(integer_casts)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::integer_cast_extras::AVAILABLE,
                )
            }
            UnstableFeature::integer_casts => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::integer_casts::AVAILABLE)
            }
            UnstableFeature::iter_array_chunks => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::iter_array_chunks::AVAILABLE)
            }
            UnstableFeature::iter_next_chunk => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::iter_next_chunk::AVAILABLE)
            }
            UnstableFeature::iter_map_windows => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::iter_map_windows::AVAILABLE)
            }
            UnstableFeature::iterator_try_collect => {
                unstable(ac, &feature, allowed, None);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::iterator_try_collect::AVAILABLE,
                )
            }
            UnstableFeature::negative_impls => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::negative_impls::AVAILABLE)
            }
            UnstableFeature::never_type => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::never_type::AVAILABLE)
            }
            UnstableFeature::os_str_split_at => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::os_str_split_at::AVAILABLE)
            }
            UnstableFeature::path_absolute_method => {
                unstable(ac, &feature, allowed, None);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::path_absolute_method::AVAILABLE,
                )
            }
            UnstableFeature::proc_macro_diagnostic => {
                unstable(ac, &feature, allowed, Some("extern crate proc_macro;"));
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::proc_macro_diagnostic::AVAILABLE,
                )
            }
            UnstableFeature::strip_circumfix => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::strip_circumfix::AVAILABLE)
            }
            UnstableFeature::try_blocks => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::try_blocks::AVAILABLE)
            }
            UnstableFeature::try_blocks_heterogeneous => {
                unstable(ac, &feature, allowed, None);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::try_blocks_heterogeneous::AVAILABLE,
                )
            }
            UnstableFeature::try_trait_v2 => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::try_trait_v2::AVAILABLE)
            }
            UnstableFeature::try_trait_v2_residual => {
                unstable(ac, &feature, allowed, None);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::try_trait_v2_residual::AVAILABLE,
                )
            }
            UnstableFeature::unsized_const_params => {
                let extra_lines = if unstable(
                    ac,
                    &UnstableFeature::adt_const_params,
                    allowed_features.includes(&UnstableFeature::adt_const_params),
                    None,
                ) {
                    Some("#![feature(adt_const_params)]\n#![allow(incomplete_features)]")
                } else {
                    None
                };
                unstable(ac, &feature, allowed, extra_lines);
                has(
                    ac,
                    &feature,
                    allowed,
                    probes::unsized_const_params::AVAILABLE,
                )
            }
            UnstableFeature::write_all_vectored => {
                unstable(ac, &feature, allowed, None);
                has(ac, &feature, allowed, probes::write_all_vectored::AVAILABLE)
            }
            UnstableFeature::OtherFeature(_) => {
                unstable(ac, &feature, allowed, None);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use super::UnstableFeature::*;
    use super::*;

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
