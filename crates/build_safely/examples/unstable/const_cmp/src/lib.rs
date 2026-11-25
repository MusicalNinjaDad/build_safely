#![cfg_attr(unstable_const_cmp, feature(const_cmp))]
#![cfg_attr(unstable_const_trait_impl, feature(const_trait_impl))]
#![allow(unused)]

// see https://github.com/rust-lang/rust/issues/162802
#[cfg(has_const_cmp)]
mod has;

#[cfg(not(has_const_cmp))]
mod has_not {
    /// ```compile_fail
    /// const fn comparisons(lhs: u8, rhs: u8) -> bool {
    ///     lhs.ge(&rhs);
    ///     lhs.gt(&rhs);
    ///     lhs.le(&rhs);
    ///     lhs.lt(&rhs);
    ///     lhs.ne(&rhs);
    ///     lhs.eq(&rhs)
    /// }
    ///
    ///     const TRUE: bool = comparisons(5, 5);
    ///     const {
    ///         assert!(TRUE);
    ///     }
    ///  ```
    fn doctest() {}
}
