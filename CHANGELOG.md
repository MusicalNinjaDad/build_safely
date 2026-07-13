# build_safely CHANGELOG

## [v0.5.3]

### New features

- Added `UnstableFeature::{adt_const_params, doc_notable_trait, strip_circumfix, unsized_const_params}`
- Added test suite validating all `UnstableFeature`s across multiple versions

## [v0.5.2]

### New features

- Added `UnstableFeature::bool_to_result`

## [v0.5.1]

### New features

- Added `Nightly::emit_unstable_feature_bundle`
- `Nightly::emit_unstable_feature` returns `bool` representing `has_...`

## [v0.5.0]

### Breaking changes

- renamed override to match new crate naming: `BUILD_SAFELY_CARGO_CONFIG_DIR`

## [v0.4.2]

### Technical changes

- renamed from `ninja-build_rs`
- updated original crate to transparent shim
- moved to dedicated repo, cleaned history with `git filter-repo`
- implemented github immutable releases
