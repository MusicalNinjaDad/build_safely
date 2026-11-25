#![cfg_attr(unstable_const_destruct, feature(const_destruct))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_destruct)]
mod has;

#[cfg(not(has_const_destruct))]
mod has_not {
    /// ```compile_fail
    /// use std::marker::Destruct;
    ///
    /// struct Foo;
    ///
    /// impl Foo {
    ///     const fn wibble<N>(val: N) -> N where
    ///         N: [const] Destruct,
    ///     {
    ///         val
    ///     }
    /// }
    ///
    ///     const FOO_5: u8 = Foo::wibble(5_u8);
    ///     const {
    ///         assert!(FOO_5 == 5_u8);
    ///     }
    ///  ```
    fn doctest() {}
}
