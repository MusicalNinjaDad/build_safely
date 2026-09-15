#![cfg_attr(unstable_default_field_values, feature(default_field_values))]
#![allow(unused)]

// Appears to be similarly affected by https://github.com/rust-lang/rust/issues/162802
#[cfg(has_default_field_values)]
mod has;

#[cfg(not(has_default_field_values))]
mod has_not {
    /// ```compile_fail
    /// struct Counter {
    ///     inner: usize = 1,
    /// }
    /// ```
    fn doctest() {}
}
