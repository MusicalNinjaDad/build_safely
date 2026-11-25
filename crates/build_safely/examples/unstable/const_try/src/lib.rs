#![cfg_attr(unstable_const_try, feature(const_try))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_try)]
mod has;

#[cfg(not(has_const_try))]
mod has_not {
    /// ```compile_fail
    /// const fn foo(maybe: Option<usize>) -> Option<usize> {
    ///     let x = maybe?;
    ///     Some(x + 1)
    /// }
    ///
    /// #[test]
    /// fn has() {
    ///     const FIVE: Option<usize> = foo(Some(4));
    ///     assert_eq!(FIVE, Some(5));
    /// }
    ///  ```
    fn doctest() {}
}
