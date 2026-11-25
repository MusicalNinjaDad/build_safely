#![cfg_attr(unstable_fn_traits, feature(fn_traits))]
#![allow(unused)]

#[cfg(has_fn_traits)]
mod has;

#[cfg(not(has_fn_traits))]
mod has_not {
    /// ```compile_fail
    /// use core::cmp::min;
    ///
    ///     let nums = (1, 2);
    ///     let small = min.call(nums);
    ///     assert_eq!(small, 1);
    /// ```
    fn doctest() {}
}
