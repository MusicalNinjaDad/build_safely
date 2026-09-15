#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_trait_impl)]
mod has;

#[cfg(not(has_const_trait_impl))]
mod has_not {
    /// ```compile_fail
    /// struct Thing2;
    /// const trait New2 {}
    ///
    /// const impl New2 for Thing2 {}
    /// ```
    fn doctest() {}
}
