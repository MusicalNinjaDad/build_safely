#![cfg_attr(unstable_const_convert, feature(const_convert))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_convert)]
mod has;

#[cfg(not(has_const_convert))]
mod has_not {
    /// ```compile_fail
    /// struct Foo(usize);
    /// impl Foo {
    ///     const fn new<N>(val: N) -> Self where
    ///         usize: [const] From<N>,
    ///     {
    ///         Self(val.into())
    ///     }
    /// }
    ///
    ///     const FOO_5: Foo = Foo::new(5_u8);
    ///     const {
    ///         assert!(FOO_5.0 == 5);
    ///     }
    ///  ```
    fn doctest() {}
}
