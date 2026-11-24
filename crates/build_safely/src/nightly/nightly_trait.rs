use super::{AllowedFeatures, UnstableFeature};

/// Adds [`AutoCfg::emit_unstable_feature`](Nightly::emit_unstable_feature)
pub trait Nightly {
    /// Offers at least 2 cfg flags for all [known features](UnstableFeature)
    ///
    /// # Feature enablement: `cfg(unstable_...)`
    ///
    /// - To be used at top-level crate via `#![cfg_attr(unstable_foo, feature(foo))]`
    ///
    /// # Cfg-gating: `cfg(has_...)`
    ///
    /// - **Do not rely on `#[cfg(not(unstable_foo))]` to suggest that `feature(foo)` is stable!**
    /// - There are 3 reasons that `#[cfg(unstable_foo)]` could be `false`:
    ///     1. The build is using `stable`/`beta` or the feature is not on the `allow-features` whitelist
    ///     2. The feature has been stabilised
    ///     3. The compiler is from before the feature was implemented
    /// - All [known features](UnstableFeature) have a `#[cfg(has_...)]` for this purpose.
    ///
    /// # Note
    ///
    /// - You must pass a set of [AllowedFeatures], created by calling [cargo_allowed_features]
    /// - If you need to test that a feature is available in order to cfg-gate your code and it is not
    ///   on the list of [known features](UnstableFeature), please raise a PR with a suggested probe.
    /// - Returns `true` if `has_...` has been set. This means `OtherFeature` will always return `false`
    /// - Will use edition 2024 for probes, unless you have specifically set an edition with [AutoCfg::set_edition]
    fn emit_unstable_feature(
        &mut self,
        feature: UnstableFeature,
        allowed_features: &AllowedFeatures,
    ) -> bool;

    /// Calls [`emit_unstable_feature`](Nightly::emit_unstable_feature) for all given features,
    /// additionally setting `has_{bundlename}` & returning `true` if all features are available.
    ///
    /// # Note
    ///
    /// - This will always return false if any of the features are
    ///   [`OtherFeature`](UnstableFeature::OtherFeature)
    fn emit_unstable_feature_bundle<F: IntoIterator<Item = UnstableFeature>>(
        &mut self,
        features: F,
        allowed_features: &AllowedFeatures,
        bundle_name: &str,
    ) -> bool {
        let cfg = format!("has_{bundle_name}");
        autocfg::emit_possibility(&cfg);

        let mut has = true;
        for feature in features.into_iter() {
            has = self.emit_unstable_feature(feature, allowed_features) && has;
        }

        if has {
            autocfg::emit(&cfg);
        }

        has
    }
}
