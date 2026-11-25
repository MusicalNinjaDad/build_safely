#![cfg_attr(unstable_const_result_trait_fn, feature(const_result_trait_fn))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_result_trait_fn)]
mod has;

#[cfg(not(has_const_result_trait_fn))]
mod has_not {
    /// ```compile_fail
    /// const fn foo() -> bool {
    ///     Ok::<_,()>(5).is_ok_and(oh_yes)
    /// }
    ///
    /// const fn oh_yes(_: usize) -> bool {
    ///     true
    /// }
    ///
    /// #[test]
    /// fn has() {
    ///     const FIVE: bool = foo();
    ///     const {
    ///         assert!(FIVE);
    ///     }
    /// }
    ///  ```
    fn doctest() {}
}
