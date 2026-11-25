#![cfg_attr(unstable_const_option_ops, feature(const_option_ops))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_option_ops)]
mod has;

#[cfg(not(has_const_option_ops))]
mod has_not {
    /// ```compile_fail
    /// const fn foo() -> bool {
    ///     Some(5).is_some_and(oh_yes)
    /// }
    ///
    /// const fn oh_yes(_: usize) -> bool {
    ///     true
    /// }
    ///
    ///     const FIVE: bool = foo();
    ///     const {
    ///         assert!(FIVE);
    ///     }
    ///  ```
    fn doctest() {}
}
