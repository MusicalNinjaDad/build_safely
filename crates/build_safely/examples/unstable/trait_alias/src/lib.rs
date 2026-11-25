#![cfg_attr(unstable_trait_alias, feature(trait_alias))]
#![allow(unused)]

#[cfg(has_trait_alias)]
mod has;

#[cfg(not(has_trait_alias))]
mod has_not {
    /// ```compile_fail
    /// #![allow(clippy::assertions_on_constants)]
    /// trait GoodError = core::error::Error + Send + Sync + 'static;
    ///
    ///     assert!(true)
    /// ```
    fn doctest() {}
}
