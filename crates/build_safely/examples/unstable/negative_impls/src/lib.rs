#![cfg_attr(unstable_negative_impls, feature(negative_impls))]
#![allow(unused)]

#[cfg(has_negative_impls)]
mod has;

#[cfg(not(has_negative_impls))]
mod has_not {
    /// ```compile_fail
    /// struct Prisoner;
    /// impl !Send for Prisoner {}
    /// ```
    fn doctest() {}
}
